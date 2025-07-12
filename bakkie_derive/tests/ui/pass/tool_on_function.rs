use bakkie_derive::tool;
use bakkie::provisions::tools::ToolError;

#[tool]
async fn my_function(param: String) -> Result<String, ToolError> {
    Ok(param)
}

fn main() {}
