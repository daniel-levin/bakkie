use bakkie_derive::tool;

#[tool]
async fn my_tool(#[app] app: (String, i32)) -> Result<String, String> {
    Ok("test".to_string())
}

fn main() {}