use schemars::Schema;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ToolParticulars {
    pub name: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub input_schema: Schema,
    pub output_schema: Option<Schema>,
}

#[derive(Debug)]
pub struct Tool {
    pub particulars: ToolParticulars,
}

#[derive(Debug, Default)]
pub struct Tools {
    tools: HashMap<String, Tool>,
}

impl Tools {
    pub fn insert_tool(&mut self, name: &str, tool: Tool) {
        self.tools.insert(name.to_owned(), tool);
    }
}
