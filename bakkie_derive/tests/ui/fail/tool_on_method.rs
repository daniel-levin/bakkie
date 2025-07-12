use bakkie_derive::tool;

struct MyStruct;

impl MyStruct {
    #[tool]
    async fn my_method(&self, param: String) -> Result<String, String> {
        Ok(param)
    }
}

fn main() {}