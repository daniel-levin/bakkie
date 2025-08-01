use bakkie_derive::tool;

#[tool]
async fn my_tool(#[app] app: App<'static>) -> Result<String, String> {
    Ok("test".to_string())
}

fn main() {}