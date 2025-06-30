#[bakkie::prompt("thing")]
fn sha256(name: String) {
    todo!();
}

#[bakkie::tool("sha256")]
async fn find_last_ten_logs(name: String) {}

use bakkie::Conversation;

#[tokio::main]
async fn main() {
    let mut c = Conversation::from_stdio();
    let mut i = 0;
    while let Some(k) = c.run().await {
        i += 1;
        let file = format!("{}.json", i);
        std::fs::write(file, format!("{:#?}", k)).unwrap();
    }
}
