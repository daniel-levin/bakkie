use crate::{
    framing::{Frame, McpFraming, Msg, Notification, Request, Response, Transport},
    proto::CodecError,
    tools::Tools,
};
use futures::{
    SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use std::sync::Arc;
use thiserror::Error;
use tokio::{
    sync::mpsc,
    task::{JoinError, JoinHandle},
};
use tokio_util::{codec::Framed, sync::CancellationToken};

#[derive(Debug, Error)]
pub enum McpServerError {
    #[error("cancelled")]
    Cancelled,

    #[error(transparent)]
    JoinError(#[from] JoinError),

    #[error("error processing incoming messages")]
    InboxError(#[from] InboxError),

    #[error("error sending outgoing messages")]
    OutboxError(#[from] OutboxError),
}

impl McpServerError {
    pub fn is_cancellation(&self) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct McpServer {
    tools: Arc<Tools>,

    inbox_task: JoinHandle<Result<(), InboxError>>,
    outbox_task: JoinHandle<Result<(), OutboxError>>,
}

impl McpServer {
    pub fn new<T: Transport>(t: T) -> Self {
        Self::new_with_tools(t, Tools::default())
    }

    pub fn new_with_tools<T: Transport>(t: T, tools: Tools) -> Self {
        let framing = t.into_framed();
        let tools = Arc::new(tools);

        let (write, read) = framing.split();

        let (tx, rx) = mpsc::unbounded_channel();

        let init_phase = InitPhase {
            tx,
            stream: read,
            tools: tools.clone(),
        };

        let outbox = Outbox {
            queue: rx,
            sink: write,
        };

        let inbox_task = tokio::task::spawn(Box::pin(async move {
            let op_phase = init_phase.negotiate().await?;

            op_phase.run_until_client_disconnects().await?;

            Ok(())
        }));

        let outbox_task =
            tokio::task::spawn(Box::pin(async move { outbox.run_to_completion().await }));

        Self {
            tools,
            inbox_task,
            outbox_task,
        }
    }

    pub async fn run(self) -> Result<(), McpServerError> {
        tokio::select! {
            maybe_faulted_in_inbox = self.inbox_task => {
                maybe_faulted_in_inbox??;
                Ok(())
            },
            maybe_faulted_in_outbox = self.outbox_task => {
                maybe_faulted_in_outbox??;
                Ok(())
            }
        }
    }
}

static CANNED_HANDSHAKE: &str = r#"
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": "2025-06-18",
    "capabilities": {
      "logging": {},
      "prompts": {
        "listChanged": false
      },
      "resources": {
        "subscribe": false,
        "listChanged": false
      },
      "tools": {
        "listChanged": false
      }
    },
    "serverInfo": {
      "name": "bakkie",
      "version": "1.0.0"
    },
    "instructions": "Nothing yet"
  }
}
"#;

#[derive(Debug, Error)]
pub enum InboxError {
    #[error("initialization phase error")]
    InitPhase(#[from] InitPhaseError),

    #[error("operation phase error")]
    OpPhase(#[from] OpPhaseError),
}

#[derive(Debug, Error)]
pub enum InitPhaseError {
    #[error("premature stream closure")]
    PrematureStreamClosure,

    #[error("cannot decode frame")]
    CannotDecodeFrame(#[from] CodecError),

    #[error("init phase requires single json-rpc request but received something else")]
    SingleRpcExpected,

    #[error("non-init message with method {0} received")]
    NonInitReceived(String),

    #[error("non-conformant init message")]
    NonConformantInitMessage,
}

#[derive(Debug)]
struct InitPhase<T: Transport> {
    tx: mpsc::UnboundedSender<Frame>,
    stream: SplitStream<Framed<T, McpFraming>>,
    tools: Arc<Tools>,
}

impl<T: Transport> InitPhase<T> {
    async fn negotiate(mut self) -> Result<OpPhase<T>, InitPhaseError> {
        let Some(rcv) = self.stream.next().await else {
            return Err(InitPhaseError::PrematureStreamClosure);
        };

        let Frame::Single(Msg::Request(Request {
            method, params, id, ..
        })) = rcv?
        else {
            return Err(InitPhaseError::SingleRpcExpected);
        };

        if method != "initialize" {
            return Err(InitPhaseError::NonInitReceived(method));
        }

        let Ok(init_msg) =
            serde_json::from_value::<bakkie_schema::V20250618::InitializeRequestParams>(params)
        else {
            return Err(InitPhaseError::NonConformantInitMessage);
        };

        let mut resp: Response = serde_json::from_str(CANNED_HANDSHAKE).unwrap();

        resp.id = id;

        let _ = self.tx.send(Frame::Single(Msg::Response(resp)));

        while let Some(Ok(Frame::Single(could_be_init))) = self.stream.next().await {
            match could_be_init {
                Msg::Request(Request { id, method, .. }) => {
                    if method == "ping" {
                        tracing::debug!("got initialization phase ping. responding");
                        let mut pong: Response = serde_json::from_str(
                            r#"
                        {
                          "jsonrpc": "2.0",
                          "id": "123",
                          "result": {}
                        }"#,
                        )
                        .unwrap();

                        pong.id = id;

                        let _ = self.tx.send(Frame::Single(Msg::Response(pong)));
                    } else {
                    }
                }
                Msg::Notification(Notification { method, .. }) => {
                    if method == "notifications/initialized" {
                        tracing::debug!("client sent init notification");
                        break;
                    }
                }
                _ => {}
            }
        }

        Ok(OpPhase {
            tx: self.tx,
            stream: self.stream,
            tools: self.tools,
        })
    }
}

#[derive(Debug, Error)]
pub enum OpPhaseError {}

#[derive(Debug)]
struct OpPhase<T: Transport> {
    tx: mpsc::UnboundedSender<Frame>,
    stream: SplitStream<Framed<T, McpFraming>>,
    tools: Arc<Tools>,
}

impl<T: Transport> OpPhase<T> {
    async fn run_until_client_disconnects(mut self) -> Result<(), OpPhaseError> {
        Ok(())
    }
}

/*
async fn rx_loop<T: Transport>(
    ct: CancellationToken,
    tx: mpsc::UnboundedSender<Frame>,
    mut stream: SplitStream<Framed<T, McpFraming>>,
    tools: Arc<Tools>,
) -> Result<(), RxLoopError> {
    tracing::debug!("awaiting initialize message");

    let Some(rcv) = stream.next().await else {
        return Err(RxLoopError::StreamClosed);
    };

    let Ok(frame) = rcv else {
    };

    let Ok(Frame::Single(Msg::Request(Request {
        method, params, id, ..
    }))) = rcv
    else {
        tracing::error!("did not receive expected initialize message");
        ct.cancel();
    };

    if method != "initialize" {
        tracing::error!("unexpected method call {method}");
        ct.cancel();
    }

    let Ok(init_msg) =
        serde_json::from_value::<bakkie_schema::V20250618::InitializeRequestParams>(params)
    else {
        tracing::error!("could not understand initialize message");
        ct.cancel();
    };

    tracing::debug!(
        "client {}@{} requests protocol version {}",
        init_msg.client_info.name,
        init_msg.client_info.version,
        init_msg.protocol_version
    );

    let mut resp: Response = serde_json::from_str(CANNED_HANDSHAKE).unwrap();

    resp.id = id;

    let _ = tx.send(Frame::Single(Msg::Response(resp)));

    tracing::debug!("responded with server hello");

    while let Some(Ok(Frame::Single(could_be_init))) = stream.next().await {
        match could_be_init {
            Msg::Request(Request { id, method, .. }) => {
                if method == "ping" {
                    tracing::debug!("got initialization phase ping. responding");
                    let mut pong: Response = serde_json::from_str(
                        r#"
                        {
                          "jsonrpc": "2.0",
                          "id": "123",
                          "result": {}
                        }"#,
                    )
                    .unwrap();

                    pong.id = id;

                    let _ = tx.send(Frame::Single(Msg::Response(pong)));
                } else {
                    tracing::error!("got request in initialization phase that is not ping");
                    ct.cancel();
                }
            }
            Msg::Notification(Notification { method, .. }) => {
                if method == "notifications/initialized" {
                    tracing::debug!("client sent init notification");
                    break;
                }
            }
            _ => {
                tracing::error!("got non-ping, also not an initialized notification");
                ct.cancel();
            }
        }
    }

    tracing::info!("handshake complete");

    while let Some(maybe_frame) = stream.next().await {
        match maybe_frame {
            Ok(frame) => {
                tracing::trace!("rx {frame:#?}");
                for msg in frame.into_messages() {
                    tokio::task::spawn(ct.clone().run_until_cancelled_owned(handle_message(
                        msg,
                        tools.clone(),
                        tx.clone(),
                    )));
                }
            }
            Err(e) => {
                tracing::error!("error in wire protocol {e:#?}");
                ct.cancel();
            }
        }
    }

    if !ct.is_cancelled() {
        tracing::debug!("rx loop ended without errors");
        ct.cancel();
    }

    Ok(())
}
*/

#[derive(Debug, Error)]
pub enum OutboxError {}

#[derive(Debug)]
struct Outbox<T: Transport> {
    queue: mpsc::UnboundedReceiver<Frame>,
    sink: SplitSink<Framed<T, McpFraming>, Frame>,
}

impl<T: Transport> Outbox<T> {
    async fn run_to_completion(mut self) -> Result<(), OutboxError> {
        while let Some(msg) = self.queue.recv().await {
            let _ = self.sink.send(msg).await;
        }

        Ok(())
    }
}

async fn handle_message(msg: Msg, tools: Arc<Tools>, tx: mpsc::UnboundedSender<Frame>) {
    match msg {
        Msg::Request(Request {
            id, method, params, ..
        }) => match method.as_str() {
            "tools/list" => {
                let _ = tx.send(Frame::Single(Msg::Response(Response {
                    jsonrpc: monostate::MustBe!("2.0"),
                    id,
                    result: serde_json::to_value(tools.as_wire()).unwrap(),
                })));
            }
            _ => {}
        },
        Msg::Error(_) => {}
        Msg::Notification(_) => {}
        Msg::Response(_) => {}
    }
}
