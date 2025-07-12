use bakkie_derive::tool;

#[tool(name="foo", description="bar", title="baz")]
async fn kapiche() -> Result<(), ()> {
    Ok(())
}

fn main() {}
