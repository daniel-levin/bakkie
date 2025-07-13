use bakkie_derive::tool;

#[tool(name = "sync_tool", description = "A sync tool")]
fn sync_with_args(param: String, count: u32) -> String {
    format!("{}: {}", param, count)
}

fn main() {}