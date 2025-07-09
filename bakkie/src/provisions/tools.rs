use schemars::Schema;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ToolParticulars {
    name: String,
    title: Option<String>,
    description: Option<String>,
    input_schema: Schema,
    output_schema: Option<Schema>,
}

#[derive(Debug)]
pub struct Tool {
    particulars: ToolParticulars,
}

#[derive(Debug, Default)]
pub struct Tools {
    tools: HashMap<String, Tool>,
}
