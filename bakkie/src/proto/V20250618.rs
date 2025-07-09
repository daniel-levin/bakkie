use crate::framing::{Frame, McpFraming, Msg, Notification, Request, Response, Transport};
use futures::{
    SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use thiserror::Error;
use tokio::sync::mpsc;
use tokio_util::{codec::Framed, sync::CancellationToken};

#[derive(Debug, Error)]
pub enum McpServerError {
    #[error("cancelled")]
    Cancelled,
}

impl McpServerError {
    pub fn is_cancellation(&self) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct McpServer {
    ct: CancellationToken,
}

impl McpServer {
    pub fn new<T: Transport>(t: T) -> Self {
        let framing = t.into_framed();
        let ct = CancellationToken::new();

        let (write, read) = framing.split();

        let (tx, rx) = mpsc::unbounded_channel();

        tokio::task::spawn(
            ct.clone()
                .run_until_cancelled_owned(rx_loop(ct.clone(), tx, read)),
        );
        tokio::task::spawn(
            ct.clone()
                .run_until_cancelled_owned(tx_loop(ct.clone(), rx, write)),
        );

        Self { ct }
    }

    pub async fn run(self) -> Result<(), McpServerError> {
        self.ct.cancelled().await;

        Err(McpServerError::Cancelled)
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
        "listChanged": true
      },
      "resources": {
        "subscribe": true,
        "listChanged": true
      },
      "tools": {
        "listChanged": true
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

async fn rx_loop<T: Transport>(
    ct: CancellationToken,
    tx: mpsc::UnboundedSender<Frame>,
    mut stream: SplitStream<Framed<T, McpFraming>>,
) {
    tracing::debug!("awaiting initialize message");

    let Some(Ok(Frame::Single(Msg::Request(Request {
        method, params, id, ..
    })))) = stream.next().await
    else {
        tracing::error!("did not receive expected initialize message");
        ct.cancel();
        return;
    };

    if method != "initialize" {
        tracing::error!("unexpected method call {method}");
        ct.cancel();
        return;
    }

    let Ok(init_msg) =
        serde_json::from_value::<bakkie_schema::V20250618::InitializeRequestParams>(params)
    else {
        tracing::error!("could not understand initialize message");
        ct.cancel();
        return;
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
                    return;
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
                return;
            }
        }
    }

    tracing::info!("handshake complete");

    while let Some(maybe_frame) = stream.next().await {
        match maybe_frame {
            Ok(frame) => {
                tracing::trace!("rx {frame:#?}");
                for msg in frame.into_messages() {
                    tokio::task::spawn(
                        ct.clone()
                            .run_until_cancelled_owned(handle_message(msg, tx.clone())),
                    );
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
}

async fn tx_loop<T: Transport>(
    _ct: CancellationToken,
    mut outbox: mpsc::UnboundedReceiver<Frame>,
    mut sink: SplitSink<Framed<T, McpFraming>, Frame>,
) {
    while let Some(msg) = outbox.recv().await {
        tracing::trace!("sending {msg:#?}");
        let _ = sink.send(msg).await;
    }
}

async fn handle_message(msg: Msg, _tx: mpsc::UnboundedSender<Frame>) {
    match msg {
        Msg::Request(_) => {}
        Msg::Error(_) => {}
        Msg::Notification(_) => {}
        Msg::Response(_) => {}
    }
}
