use bakkie_derive::tool;
use bakkie::provisions::tools::ToolError;

#[tool(name="foo", description="bar", title="baz")]
async fn kapiche() -> Result<(), ToolError> {
    Ok(())
}

fn main() {}
