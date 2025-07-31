use bakkie_derive::tool;
use bakkie::provisions::tools::ToolError;

#[tool]
async fn my_tool(#[app] _my_app: bakkie::proto::V20250618::App<()>, name: String) -> Result<String, ToolError> {
    Ok(format!("Hello, {}", name))
}

fn main() {}