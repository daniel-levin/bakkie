use bakkie_derive::tool;

#[tool(,)]
async fn my_function(param: String) -> Result<String, String> {
    Ok(param)
}

fn main() {}