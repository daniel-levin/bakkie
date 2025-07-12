use bakkie_derive::tool;

const MY_NAME: &str = "test";

#[tool(name = MY_NAME)]
async fn my_function(param: String) -> Result<String, String> {
    Ok(param)
}

fn main() {}