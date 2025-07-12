use bakkie_derive::tool;

#[tool(name = 123)]
async fn my_function(param: String) -> Result<String, String> {
    Ok(param)
}

fn main() {}