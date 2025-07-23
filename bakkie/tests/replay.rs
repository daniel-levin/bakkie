use bakkie::{framing::Transport, proto::V20250618::McpServer, provisions::Provisions};
use futures::{SinkExt, stream::StreamExt};
use tokio::io::AsyncWriteExt;

macro_rules! slurp {
    ($file:literal) => {
        serde_json::from_slice(include_bytes!($file)).unwrap()
    };
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn replay_claude_code_interaction() -> anyhow::Result<()> {
    bakkie::dnp!();

    let (client, server) = tokio::io::duplex(64);

    let exit = tokio::task::spawn(async move {
        let provisions = Provisions::default();

        let server = McpServer::new_with_provisions(server, provisions);

        server.run().await
    });

    let mut framed = client.into_framed();

    framed.send(slurp!("../testdata/claude_hello.json")).await?;

    let server_hello = framed.next().await.unwrap()?;

    framed
        .send(slurp!("../testdata/claude_notify.json"))
        .await?;

    framed
        .send(slurp!("../testdata/claude_ask_for_tools.json"))
        .await?;

    let list_of_tools = framed.next().await.unwrap()?;

    drop(framed);
    let _ = exit.await?;

    Ok(())
}
