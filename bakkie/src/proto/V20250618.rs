use crate::{
    framing::{Frame, McpFraming, Msg, RequestId, RequestOrNotification, Response, Transport},
    proto::CodecError,
    provisions::{Provisions, tools::ToolInput},
};
use futures::{
    SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use std::sync::Arc;
use thiserror::Error;
use tokio::{
    sync::{RwLock, mpsc},
    task::{JoinError, JoinHandle},
};
use tokio_util::codec::Framed;

#[derive(Debug, Error)]
pub enum McpServerError {
    #[error(transparent)]
    JoinError(#[from] JoinError),

    #[error("error processing incoming messages")]
    InboxError(#[from] InboxError),

    #[error("error sending outgoing messages")]
    OutboxError(#[from] OutboxError),

    #[error("both inbox and outbox failed")]
    BothFailed {
        inbox_error: InboxError,
        outbox_error: OutboxError,
    },
}

#[derive(Debug, Default)]
pub struct App<A = ()>
where
    A: Send + Sync + 'static,
{
    interior: Arc<RwLock<A>>,
}

impl<A> App<A>
where
    A: Send + Sync + 'static,
{
    pub fn new(a: A) -> Self {
        Self {
            interior: Arc::new(RwLock::new(a)),
        }
    }
}

#[derive(Debug)]
pub struct McpServer<A = ()>
where
    A: Send + Sync + 'static,
{
    #[allow(dead_code)]
    inbox_task: JoinHandle<Result<(), InboxError>>,
    outbox_task: JoinHandle<Result<(), OutboxError>>,
    provisions: Provisions,
    app: App<A>,
}

impl<A> McpServer<A>
where
    A: Send + Sync + 'static,
{
    pub fn new_with_provisions_and_application<T: Transport>(
        t: T,
        provisions: Provisions,
        app: A,
    ) -> Self {
        let framing = t.into_framed();

        let (write, read) = framing.split();

        let (tx, rx) = mpsc::unbounded_channel();

        let init_phase = InitPhase {
            tx,
            stream: read,
            provisions: provisions.clone(),
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
            provisions,
            inbox_task,
            outbox_task,
            app: App::new(app),
        }
    }

    pub async fn run(self) -> Result<(), McpServerError> {
        let (inbox_result, outbox_result) = tokio::join!(self.inbox_task, self.outbox_task);

        match (inbox_result?, outbox_result?) {
            (Ok(()), Ok(())) => Ok(()),
            (Err(inbox_err), Ok(())) => Err(McpServerError::InboxError(inbox_err)),
            (Ok(()), Err(outbox_err)) => Err(McpServerError::OutboxError(outbox_err)),
            (Err(inbox_err), Err(outbox_err)) => Err(McpServerError::BothFailed {
                inbox_error: inbox_err,
                outbox_error: outbox_err,
            }),
        }
    }
}

impl McpServer<()> {
    pub fn new<T: Transport>(t: T) -> Self {
        Self::new_with_provisions(t, Provisions::default())
    }

    pub fn new_with_provisions<T: Transport>(t: T, provisions: Provisions) -> Self {
        Self::new_with_provisions_and_application(t, provisions, ())
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

    #[error("received non-ping while awaiting init notification")]
    ReceivedNonPing,
}

#[derive(Debug)]
struct InitPhase<T: Transport> {
    tx: mpsc::UnboundedSender<Frame>,
    stream: SplitStream<Framed<T, McpFraming>>,
    provisions: Provisions,
}

impl<T: Transport> InitPhase<T> {
    async fn negotiate(mut self) -> Result<OpPhase<T>, InitPhaseError> {
        let Some(rcv) = self.stream.next().await else {
            return Err(InitPhaseError::PrematureStreamClosure);
        };

        let Frame::Single(Msg::Request(RequestOrNotification::Request {
            method,
            params: Some(params),
            id,
            ..
        })) = rcv?
        else {
            return Err(InitPhaseError::SingleRpcExpected);
        };

        if method != "initialize" {
            return Err(InitPhaseError::NonInitReceived(method));
        }

        let Ok(_init_msg) =
            serde_json::from_value::<bakkie_schema::V20250618::InitializeRequestParams>(params)
        else {
            return Err(InitPhaseError::NonConformantInitMessage);
        };

        let mut resp: Response = serde_json::from_str(CANNED_HANDSHAKE).unwrap();

        resp.id = id;

        let _ = self.tx.send(Frame::Single(Msg::Response(resp)));

        while let Some(Ok(Frame::Single(could_be_init))) = self.stream.next().await {
            match could_be_init {
                Msg::Request(RequestOrNotification::Request { id, method, .. }) => {
                    if method == "ping" {
                        let pong = Response {
                            jsonrpc: monostate::MustBe!("2.0"),
                            id,
                            result: serde_json::Map::new().into(),
                        };

                        let _ = self.tx.send(Frame::Single(Msg::Response(pong)));
                    } else {
                        return Err(InitPhaseError::ReceivedNonPing);
                    }
                }
                Msg::Request(RequestOrNotification::Notification { method, .. }) => {
                    if method == "notifications/initialized" {
                        break;
                    }
                }
                _ => {
                    return Err(InitPhaseError::ReceivedNonPing);
                }
            }
        }

        Ok(OpPhase {
            tx: self.tx,
            stream: self.stream,
            provisions: self.provisions,
        })
    }
}

#[derive(Debug, Error)]
pub enum OpPhaseError {}

#[derive(Debug)]
struct OpPhase<T: Transport> {
    tx: mpsc::UnboundedSender<Frame>,
    stream: SplitStream<Framed<T, McpFraming>>,
    provisions: Provisions,
}

impl<T: Transport> OpPhase<T> {
    async fn run_until_client_disconnects(mut self) -> Result<(), OpPhaseError> {
        while let Some(maybe_frame) = self.stream.next().await {
            if let Ok(frame) = maybe_frame {
                for msg in frame.into_messages() {
                    tokio::task::spawn(Box::pin(handle_message(
                        msg,
                        self.provisions.clone(),
                        self.tx.clone(),
                    )));
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum OutboxError {
    #[error(transparent)]
    Codec(#[from] CodecError),
}

#[derive(Debug)]
struct Outbox<T: Transport> {
    queue: mpsc::UnboundedReceiver<Frame>,
    sink: SplitSink<Framed<T, McpFraming>, Frame>,
}

impl<T: Transport> Outbox<T> {
    async fn run_to_completion(mut self) -> Result<(), OutboxError> {
        while let Some(msg) = self.queue.recv().await {
            self.sink.send(msg).await?;
        }

        Ok(())
    }
}

async fn handle_message(msg: Msg, provisions: Provisions, tx: mpsc::UnboundedSender<Frame>) {
    match msg {
        Msg::Request(RequestOrNotification::Request {
            id, method, params, ..
        }) => match method.as_str() {
            "tools/call" => {
                tokio::task::spawn(Box::pin(call_tool(id, params.unwrap(), provisions, tx)));
            }
            "tools/list" => {
                let _ = tx.send(Frame::Single(Msg::Response(Response {
                    jsonrpc: monostate::MustBe!("2.0"),
                    id,
                    result: serde_json::to_value(provisions.schema_tools().await.unwrap()).unwrap(),
                })));
            }

            _ => {}
        },
        Msg::Error(_) => {}
        Msg::Response(_) => {}
        _ => {}
    }
}

#[allow(unused_variables)]
async fn call_tool(
    request_id: RequestId,
    params: serde_json::Value,
    provisions: Provisions,
    tx: mpsc::UnboundedSender<Frame>,
) {
    let ctrp =
        serde_json::from_value::<bakkie_schema::V20250618::CallToolRequestParams>(params).unwrap();

    let id = request_id.clone();

    let input = ToolInput {
        request_id,
        params: ctrp.arguments,
    };

    let prepped_fut = provisions
        .prepare_tool_future(&ctrp.name, input)
        .await
        .unwrap();

    // todo: make readable.
    tokio::task::spawn(Box::pin(async move {
        match prepped_fut.await {
            Ok(tool_output) => {
                let to = tool_output.as_tool_output();
                let wire_fmt = move || serde_json::to_value(to?);

                match wire_fmt() {
                    Ok(wire) => {
                        let Ok(_) = tx.send(Frame::Single(Msg::Response(Response {
                            id,
                            jsonrpc: monostate::MustBe!("2.0"),
                            result: wire,
                        }))) else {
                            todo!();
                        };
                    }
                    Err(e) => {
                        todo!();
                    }
                }
            }
            Err(failure) => {
                todo!();
            }
        }
    }));
}
