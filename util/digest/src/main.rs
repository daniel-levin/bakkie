#[bakkie::prompt("thing")]
fn sha256(name: String) {
    todo!();
}

#[bakkie::tool("sha256")]
async fn find_last_ten_logs(name: String) {}

#[tokio::main]
async fn main() -> bakkie::Result<()> {
    Ok(())
}
