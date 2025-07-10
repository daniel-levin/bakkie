pub mod prompts;
pub mod resources;
pub mod tools;

use self::tools::{Tool, Tools};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Default)]
pub struct Provisions {
    tools: Arc<RwLock<Tools>>,
}

impl Provisions {
    pub async fn insert_tool(&self, name: &str, tool: Tool) {
        let mut tools = self.tools.write().await;
        tools.insert_tool(name, tool);
    }

    pub async fn schema_tools(
        &self,
    ) -> Result<Vec<bakkie_schema::V20250618::Tool>, serde_json::Error> {
        let tools = self.tools.read().await;
        tools.schema_tools()
    }
}
