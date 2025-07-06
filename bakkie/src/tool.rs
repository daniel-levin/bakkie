use bakkie_schema::{CallToolRequestParams, CallToolResult};
use schemars::Schema;
use serde::Serialize;
use serde_json::{Map, Value};
use std::{collections::HashMap, pin::Pin};
use thiserror::Error;

#[derive(Debug)]
pub struct ToolOutput {}

#[derive(Debug, Error)]
pub enum ToolError {}

#[derive(Debug, Serialize)]
pub struct Tool {
    pub name: String,
    pub title: String,
    pub description: String,

    #[serde(rename = "inputSchema")]
    pub input_schema: crate::schemars::Schema,

    #[serde(skip)]
    pub construct_fn: ConstructTool,
}

pub type ConstructTool = fn(Map<String, Value>) -> ToolFut;

pub type ToolFut = Pin<Box<dyn Future<Output = Result<ToolOutput, ToolError>> + Send>>;

#[derive(Debug, Default)]
pub(crate) struct Tools {
    pub registry: HashMap<String, Tool>,
}

impl Tools {
    /// Inject the request body into a future that may be awaited, if such a tool exists.
    pub fn prepare_future(&self, request: CallToolRequestParams) -> Option<ToolFut> {
        if let Some(tool) = self.registry.get(&request.name) {
            let fut = (tool.construct_fn)(request.arguments);
            Some(fut)
        } else {
            None
        }
    }
}
