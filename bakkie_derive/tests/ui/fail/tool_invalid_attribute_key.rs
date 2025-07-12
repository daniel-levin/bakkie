use bakkie_derive::tool;

#[tool(invalid_key = "value")]
async fn my_function(param: String) -> Result<String, String> {
    Ok(param)
}

fn main() {}