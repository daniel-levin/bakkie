use crate::framing::RequestId;
use schemars::Schema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, future::Future, pin::Pin};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ToolOutput(bakkie_schema::V20250618::CallToolResult);

impl Default for ToolOutput {
    fn default() -> Self {
        Self(bakkie_schema::V20250618::CallToolResult {
            content: vec![],
            is_error: None,
            meta: serde_json::Map::default(),
            structured_content: serde_json::Map::default(),
        })
    }
}

pub trait AsToolOutput: Send {
    fn as_tool_output(&self) -> ToolOutput;
}

impl AsToolOutput for () {
    fn as_tool_output(&self) -> ToolOutput {
        ToolOutput::default()
    }
}

impl AsToolOutput for usize {
    fn as_tool_output(&self) -> ToolOutput {
        self.to_string().as_tool_output()
    }
}

impl AsToolOutput for String {
    fn as_tool_output(&self) -> ToolOutput {
        ToolOutput(bakkie_schema::V20250618::CallToolResult {
            content: vec![bakkie_schema::V20250618::ContentBlock::TextContent(
                bakkie_schema::V20250618::TextContent {
                    annotations: None,
                    meta: serde_json::Map::default(),
                    text: self.clone(),
                    type_: "text".into(),
                },
            )],
            is_error: None,
            meta: serde_json::Map::default(),
            structured_content: serde_json::Map::default(),
        })
    }
}

pub type ToolFuture =
    Pin<Box<dyn Future<Output = Result<Box<dyn AsToolOutput>, ToolError>> + Send>>;

#[allow(dead_code)]
pub struct ToolInput {
    pub request_id: RequestId,
    pub params: serde_json::Map<String, serde_json::Value>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ToolParticulars {
    pub name: String,

    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: Option<String>,

    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "inputSchema")]
    pub input_schema: Schema,

    #[serde(rename = "outputSchema")]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub output_schema: Option<Schema>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaTools {
    pub tools: Vec<ToolParticulars>,
}

pub struct Tool {
    pub particulars: ToolParticulars,
    pub tool_fn: Box<dyn Fn(ToolInput) -> ToolFuture + Send + Sync>,
}

impl Tool {
    pub fn name(&self) -> &str {
        &self.particulars.name
    }
}

impl std::fmt::Debug for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tool")
            .field("particulars", &self.particulars)
            .field("tool_fn", &"<closure>")
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct Tools {
    tools: HashMap<String, Tool>,
}

impl Tools {
    pub fn insert_tool(&mut self, name: String, tool: Tool) {
        self.tools.insert(name, tool);
    }

    pub fn schema_tools(&self) -> Result<SchemaTools, serde_json::Error> {
        Ok(SchemaTools {
            tools: self
                .tools
                .values()
                .map(|tool| tool.particulars.clone())
                .collect(),
        })
    }

    pub fn get(&self, name: &str) -> Option<&Tool> {
        self.tools.get(name)
    }
}
