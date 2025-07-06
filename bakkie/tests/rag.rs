use bakkie::{McpServer, codec::Transport};
use bakkie_schema::JsonrpcMessage;

macro_rules! load {
    ($name:ident, $file:literal) => {
        static $name: std::sync::LazyLock<JsonrpcMessage> = std::sync::LazyLock::new(|| {
            let content = include_str!($file);

            serde_json::from_str(content).unwrap()
        });
    };
}

load!(INIT, "static/init.json");

#[bakkie::tool("count letters")]
fn count_letters(haystack: String, needle: char) -> bakkie::Result<String> {
    todo!();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b() -> anyhow::Result<()> {
    let (tc, ts) = tokio::io::duplex(64);

    tokio::task::spawn(async {
        let mcp = McpServer::new(Transport::new(ts));
    });

    let x = &*INIT;

    dbg!(x);

    let mut t = Transport::new(tc);

    Ok(())
}
