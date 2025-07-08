use crate::{
    Stream,
    framing::{CodecError, Frame, StdioStream, Transport},
};
use bakkie_schema::V20250618::Implementation;
use thiserror::Error;
use tokio::task::{JoinError, JoinSet};
use tokio_util::sync::CancellationToken;

use crate::{
    proto,
    proto::{HandshakeError, Mcp},
};

#[derive(Debug, Error)]
#[error("server shut down with error")]
pub struct DirtyShutdown {
    #[from]
    cause: InternalError,
}

#[derive(Debug, Error)]
enum InternalError {
    #[error("protocol error")]
    Protocol(#[from] ProtocolError),
}

#[derive(Debug, Error)]
enum ProtocolError {
    #[error("handshake error")]
    Handshake(#[from] HandshakeError),

    #[error(transparent)]
    Codec(#[from] CodecError),
}

#[derive(Debug)]
pub(crate) enum Completion {}

#[derive(Debug, Error)]
pub enum CompletionError {}

#[derive(Debug)]
pub struct McpServer<T: Stream, M: Mcp = proto::V20250618::McpServerImpl> {
    transport: Transport<T>,
    mcp: M,

    ct: CancellationToken,
    tasks: JoinSet<Result<Completion, CompletionError>>,
}

impl<T: Stream, M: Mcp> McpServer<T, M> {
    pub fn new(transport: Transport<T>, mcp: M) -> Self {
        let ct = CancellationToken::new();
        Self {
            transport,
            ct,
            mcp,
            tasks: JoinSet::new(),
        }
    }

    pub async fn run(&mut self) -> Result<(), DirtyShutdown> {
        let na = self
            .mcp
            .handshake()
            .await
            .map_err(|e| InternalError::Protocol(ProtocolError::Handshake(e)))?;

        while !self.ct.is_cancelled() {
            let rx = self.transport.rx();
            let task_finish = self.tasks.join_next();
            tokio::select! {
                frame = rx => {
                    self.mcp.on_rx(frame).await;
                }
                completion = task_finish => {
                    self.on_completion(completion).await;
                }
            }
        }

        Ok(())
    }

    /*
    async fn handshake(&mut self) -> Result<NegotiatedAgreement, HandshakeError> {
        let Some(Ok(Frame::Single(Msg::Request(Request {
            method,
            id,
            jsonrpc,
            params: Some(JsonrpcRequestParams { extra, .. }),
            ..
        })))) = self.transport.rx().await
        else {
            return Err(HandshakeError::ExpectingInitializeRequest);
        };

        if method != "initialize" {
            return Err(HandshakeError::WrongMethod { method });
        }

        let init_req: InitializeRequestParams =
            serde_json::from_value(serde_json::Value::Object(extra))?;

        tracing::trace!("rx init msg");

        let na = NegotiatedAgreement::new(&init_req.protocol_version);

        let init_resp = InitializeResult {
            capabilities: ServerCapabilities {
                tools: Some(ServerCapabilitiesTools {
                    list_changed: Some(true),
                }),
                prompts: Some(ServerCapabilitiesPrompts {
                    list_changed: Some(true),
                }),
                resources: Some(ServerCapabilitiesResources {
                    subscribe: Some(true),
                    list_changed: Some(true),
                }),
                ..Default::default()
            },
            instructions: self.instructions.clone(),
            protocol_version: na.server_requested_version.to_string(),
            server_info: self.server_info.clone(),
            meta: Default::default(),
        };

        let res = bakkie_schema::new_response(id, &init_resp)?;

        self.transport.tx(res).await?;

        tracing::trace!("tx init response");

        let Some(Ok(Frame::Single(JsonrpcMessage::Notification(JsonrpcNotification {
            method,
            ..
        })))) = self.transport.rx().await
        else {
            return Err(HandshakeError::DidNotReceiveNotification);
        };

        if method != "notifications/initialized" {
            return Err(HandshakeError::DidNotReceiveNotification);
        }

        tracing::trace!("rx notifications/initialized; handshake complete");

        Ok(na)
    }*/

    async fn on_rx(
        &mut self,
        frame: Option<Result<Frame, CodecError>>,
    ) -> Result<(), ProtocolError> {
        match frame {
            Some(Ok(frame)) => {
                self.delegate_rx(frame).await;
                Ok(())
            }
            Some(Err(e)) => Err(ProtocolError::Codec(e)),
            None => Ok(()),
        }
    }

    async fn delegate_rx(&mut self, f: Frame) {}

    async fn on_completion(
        &mut self,
        completion: Option<Result<Result<Completion, CompletionError>, JoinError>>,
    ) {
    }
}

impl McpServer<StdioStream, proto::V20250618::McpServerImpl> {
    pub fn over_stdio() -> Self {
        Self::new(
            Transport::over_stdio(),
            proto::V20250618::McpServerImpl::default(),
        )
    }
}

fn default_implementation() -> Implementation {
    Implementation {
        name: env!("CARGO_PKG_NAME").into(),
        title: None,
        version: env!("CARGO_PKG_VERSION").into(),
    }
}
