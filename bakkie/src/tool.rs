use bakkie_schema::CallToolResult;
use std::{collections::HashMap, pin::Pin};

type Tool = fn(u32) -> Pin<Box<dyn Future<Output = crate::Result<CallToolResult>> + Send>>;

#[derive(Debug)]
pub struct Tools {
    registry: HashMap<String, Tool>,
}

impl Tools {
    pub fn delegate(&self, tool: &str, serde: u32) {
        if let Some(x) = self.registry.get(tool) {
            let fut = x(1);
            tokio::task::spawn(async move {
                // admin work
                fut.await
            });
        }
    }
}
