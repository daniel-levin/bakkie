use bakkie_derive::tool;
use bakkie::provisions::tools::ToolError;

#[tool]
fn sync_function_with_result(param: String) -> Result<String, ToolError> {
    Ok(param)
}

fn main() {}