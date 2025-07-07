use crate::{
    Stream,
    framing::{CodecError, Frame, Msg, Request, StdioStream, Transport},
    proto::NegotiatedAgreement,
    tool::{Tool, Tools},
};
use bakkie_schema::{
    Implementation, InitializeRequestParams, InitializeResult, JsonrpcMessage, JsonrpcNotification,
    JsonrpcRequest, JsonrpcRequestParams, ServerCapabilities, ServerCapabilitiesPrompts,
    ServerCapabilitiesResources, ServerCapabilitiesTools,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::task::{JoinError, JoinSet};
use tokio_util::sync::CancellationToken;

use bakkie_schema::RequestId;

#[derive(Debug)]
pub struct McpServer<T: Stream> {
    transport: Transport<T>,
    ct: CancellationToken,
    tasks: JoinSet<Result<Completion, CompletionError>>,

    tools: Tools,

    server_info: Implementation,
    instructions: Option<String>,
}

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

#[derive(Debug, Error)]
enum HandshakeError {
    #[error("did not received expected 'initialize' request")]
    ExpectingInitializeRequest,

    #[error("method '{method}' called in handshake when 'initialize' was expected")]
    WrongMethod { method: String },

    #[error("noncompliant handshake received")]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    CannotAllocResponse(#[from] bakkie_schema::ResponseSerializeError),

    #[error(transparent)]
    Codec(#[from] CodecError),

    #[error("did not receive notification")]
    DidNotReceiveNotification,
}

#[derive(Debug)]
pub(crate) enum Completion {}

#[derive(Debug, Error)]
pub enum CompletionError {}

impl McpServer<StdioStream> {
    pub fn over_stdio() -> Self {
        Self::new(Transport::over_stdio())
    }
}

impl<T: Stream> McpServer<T> {
    pub fn new(transport: Transport<T>) -> Self {
        let ct = CancellationToken::new();
        Self {
            transport,
            ct,
            tasks: JoinSet::new(),
            tools: Tools::default(),
            server_info: default_implementation(),
            instructions: None,
        }
    }

    pub fn with_tool(mut self, tool: Tool) -> Self {
        self.tools.registry.insert(tool.name.clone(), tool);
        self
    }

    pub fn with_server_info(mut self, server_info: Implementation) -> Self {
        self.server_info = server_info;
        self
    }

    pub fn with_instructions(mut self, instructions: &str) -> Self {
        self.instructions = Some(instructions.into());
        self
    }

    pub async fn run(&mut self) -> Result<(), DirtyShutdown> {
        /*let na = self
        .handshake()
        .await
        .map_err(|e| InternalError::Protocol(ProtocolError::Handshake(e)))?;*/

        while !self.ct.is_cancelled() {
            let rx = self.transport.rx();
            let task_finish = self.tasks.join_next();
            tokio::select! {
                frame = rx => {
                    self.on_rx(frame).await;
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

fn default_implementation() -> Implementation {
    Implementation {
        name: env!("CARGO_PKG_NAME").into(),
        title: None,
        version: env!("CARGO_PKG_VERSION").into(),
    }
}
