use crate::framing::RequestId;
use bakkie_schema::V20250618::{Tool as SchemaTool, ToolInputSchema};
use schemars::Schema;
use std::{collections::HashMap, future::Future, pin::Pin};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

#[derive(Debug)]
pub enum ToolOutput {
    Number(usize),
}

pub trait IntoToolOutput: Send {
    fn into_tool_output(&self) -> ToolOutput;
}

impl IntoToolOutput for () {
    fn into_tool_output(&self) -> ToolOutput {
        todo!()
    }
}

pub type ToolFuture =
    Pin<Box<dyn Future<Output = Result<Box<dyn IntoToolOutput>, ToolError>> + Send>>;

#[allow(dead_code)]
pub struct ToolInput {
    pub request_id: RequestId,
    pub params: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug)]
pub struct ToolParticulars {
    pub name: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub input_schema: Schema,
    pub output_schema: Option<Schema>,
}

impl ToolParticulars {
    pub fn to_schema_tool(&self) -> Result<SchemaTool, serde_json::Error> {
        let input_schema: ToolInputSchema =
            serde_json::from_value(serde_json::to_value(&self.input_schema)?)?;

        let output_schema = if let Some(ref output_schema) = self.output_schema {
            Some(serde_json::from_value(serde_json::to_value(
                output_schema,
            )?)?)
        } else {
            None
        };

        Ok(SchemaTool {
            annotations: None,
            description: self.description.clone(),
            input_schema,
            meta: Default::default(),
            name: self.name.clone(),
            output_schema,
            title: self.title.clone(),
        })
    }
}

pub struct Tool {
    pub particulars: ToolParticulars,
    pub tool_fn: Box<dyn Fn(ToolInput) -> ToolFuture + Send + Sync>,
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
    pub fn insert_tool(&mut self, name: &str, tool: Tool) {
        self.tools.insert(name.to_owned(), tool);
    }

    pub fn schema_tools(&self) -> Result<Vec<SchemaTool>, serde_json::Error> {
        self.tools
            .values()
            .map(|tool| tool.particulars.to_schema_tool())
            .collect()
    }

    pub fn get(&self, name: &str) -> Option<&Tool> {
        self.tools.get(name)
    }
}

mod impls {
    use super::*;

    impl IntoToolOutput for usize {
        fn into_tool_output(&self) -> ToolOutput {
            ToolOutput::Number(*self)
        }
    }

    impl IntoToolOutput for String {
        fn into_tool_output(&self) -> ToolOutput {
            todo!();
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    use super::*;
    use schemars::{JsonSchema, schema_for};
    use serde::Deserialize;

    #[derive(JsonSchema, Deserialize)]
    #[allow(dead_code)]
    struct TestInput {
        name: String,
        age: u32,
    }

    #[derive(JsonSchema, Deserialize)]
    #[allow(dead_code)]
    struct TestOutput {
        result: String,
    }

    #[test]
    fn test_to_schema_tool_basic() {
        let input_schema = schema_for!(TestInput);
        let tool_particulars = ToolParticulars {
            name: "test_tool".to_string(),
            title: Some("Test Tool".to_string()),
            description: Some("A test tool".to_string()),
            input_schema,
            output_schema: None,
        };
        let schema_tool = tool_particulars.to_schema_tool().unwrap();

        assert_eq!(schema_tool.name, "test_tool");
        assert_eq!(schema_tool.title, Some("Test Tool".to_string()));
        assert_eq!(schema_tool.description, Some("A test tool".to_string()));
        assert_eq!(schema_tool.input_schema.type_, "object");
        assert!(schema_tool.output_schema.is_none());
        assert!(schema_tool.annotations.is_none());
    }

    #[test]
    fn test_to_schema_tool_with_output_schema() {
        let input_schema = schema_for!(TestInput);
        let output_schema = schema_for!(TestOutput);
        let tool_particulars = ToolParticulars {
            name: "test_tool_with_output".to_string(),
            title: None,
            description: None,
            input_schema,
            output_schema: Some(output_schema),
        };
        let schema_tool = tool_particulars.to_schema_tool().unwrap();

        assert_eq!(schema_tool.name, "test_tool_with_output");
        assert!(schema_tool.title.is_none());
        assert!(schema_tool.description.is_none());
        assert_eq!(schema_tool.input_schema.type_, "object");
        assert!(schema_tool.output_schema.is_some());
        assert_eq!(schema_tool.output_schema.unwrap().type_, "object");
    }

    #[test]
    fn test_to_schema_tool_minimal() {
        let input_schema = Schema::from(
            serde_json::json!({
                "type": "string"
            })
            .as_object()
            .unwrap()
            .clone(),
        );
        let tool_particulars = ToolParticulars {
            name: "minimal_tool".to_string(),
            title: None,
            description: None,
            input_schema,
            output_schema: None,
        };
        let schema_tool = tool_particulars.to_schema_tool().unwrap();

        assert_eq!(schema_tool.name, "minimal_tool");
        assert!(schema_tool.title.is_none());
        assert!(schema_tool.description.is_none());
        assert!(schema_tool.annotations.is_none());
        assert!(schema_tool.output_schema.is_none());
    }
}
