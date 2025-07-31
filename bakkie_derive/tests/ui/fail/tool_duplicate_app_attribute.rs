use bakkie_derive::tool;

#[tool]
async fn my_tool(#[app] app1: i32, #[app] app2: i32) -> Result<String, String> {
    Ok("test".to_string())
}

fn main() {}