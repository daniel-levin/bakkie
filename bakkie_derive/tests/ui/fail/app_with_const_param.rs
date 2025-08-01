use bakkie_derive::tool;

#[tool]
async fn my_tool(#[app] app: App<42>) -> Result<String, String> {
    Ok("test".to_string())
}

fn main() {}