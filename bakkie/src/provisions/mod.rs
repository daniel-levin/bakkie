pub mod prompts;
pub mod resources;
pub mod tools;

use self::tools::{Tool, ToolFuture, ToolInput, Tools};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Default)]
pub struct Provisions {
    tools: Arc<RwLock<Tools>>,
}

impl Provisions {
    pub async fn insert_tool(&self, tool: Tool) {
        let mut tools = self.tools.write().await;
        tools.insert_tool(tool.name().to_owned(), tool);
    }

    pub async fn schema_tools(
        &self,
    ) -> Result<Vec<bakkie_schema::V20250618::Tool>, serde_json::Error> {
        let tools = self.tools.read().await;
        tools.schema_tools()
    }

    pub async fn prepare_tool_future(
        &self,
        name: &str,
        tool_input: ToolInput,
    ) -> Option<ToolFuture> {
        let tools = self.tools.read().await;
        let tool: &Tool = tools.get(name)?;

        Some((tool.tool_fn)(tool_input))
    }
}
