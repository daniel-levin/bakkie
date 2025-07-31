pub mod prompts;
pub mod resources;
pub mod tools;

use self::tools::{Tool, ToolFuture, ToolInput, Tools};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct Provisions<A: Send + Sync + 'static> {
    tools: Arc<RwLock<Tools<A>>>,
}

impl<A: Send + Sync + 'static> Default for Provisions<A> {
    fn default() -> Self {
        Self {
            tools: Arc::new(RwLock::new(Tools::new())),
        }
    }
}

impl<A: Send + Sync + 'static> Clone for Provisions<A> {
    fn clone(&self) -> Self {
        Self {
            tools: self.tools.clone(),
        }
    }
}

impl<A: Send + Sync + 'static> Provisions<A> {
    pub async fn insert_tool<F: FnOnce() -> Tool<A>>(&self, tf: F) {
        let tool = tf();
        let mut tools = self.tools.write().await;
        tools.insert_tool(tool.name().to_owned(), tool);
    }

    pub async fn schema_tools(&self) -> Result<tools::SchemaTools, serde_json::Error> {
        let tools = self.tools.read().await;
        tools.schema_tools()
    }

    pub async fn prepare_tool_future(
        &self,
        name: &str,
        tool_input: ToolInput<A>,
    ) -> Option<ToolFuture> {
        let tools = self.tools.read().await;
        let tool: &Tool<A> = tools.get(name)?;

        Some((tool.tool_fn)(tool_input))
    }
}
