use super::{HandshakeError, Mcp, NegotiatedAgreement, RxError};
use crate::{
    framing::{CodecError, Frame, Msg},
    tool::{Tool, Tools},
};

#[derive(Debug, Default)]
pub struct McpServerImpl {
    tools: Tools,

    instructions: Option<String>,
}

impl McpServerImpl {
    pub fn with_tool(mut self, tool: Tool) -> Self {
        self.tools.registry.insert(tool.name.clone(), tool);
        self
    }

    pub fn with_instructions(mut self, instructions: &str) -> Self {
        self.instructions = Some(instructions.into());
        self
    }
}

impl Mcp for McpServerImpl {
    async fn handshake(&mut self) -> Result<NegotiatedAgreement, HandshakeError> {
        todo!();
    }

    async fn rx_frame(&mut self, frame: Frame) -> Result<(), RxError> {
        for msg in frame.into_messages() {
            match msg {
                Msg::Request(r) => {}
                Msg::Notification(r) => {}
                Msg::Response(r) => {}
                Msg::Error(r) => {}
            }
        }
        Ok(())
    }
}
