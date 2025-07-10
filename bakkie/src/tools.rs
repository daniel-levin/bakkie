use schemars::Schema;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Tools {
    tools: HashMap<String, InvocableTool>,
}

impl Tools {
    pub fn as_wire(&self) -> ToolsList<'_> {
        ToolsList {
            tools: self.tools.values().collect(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ToolsList<'a> {
    tools: Vec<&'a InvocableTool>,
}

#[derive(Debug, Serialize)]
pub struct InvocableTool {
    name: String,
    description: String,
    input_schema: Schema,
}
