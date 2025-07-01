#[bakkie::prompt("thing")]
fn sha256(name: String) {
    todo!();
}

#[bakkie::tool("sha256")]
async fn find_last_ten_logs(name: String) {}

use bakkie::Conversation;

#[tokio::main]
async fn main() -> bakkie::Result<()> {
    let mut c = Conversation::from_stdio();
    c.run_to_completion().await
}
