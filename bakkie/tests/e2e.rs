use bakkie::{
    McpServer,
    framing::{Frame, Transport},
};
use bakkie_schema::JsonrpcMessage;

/*
macro_rules! load {
    ($name:ident, $file:literal) => {
        static $name: std::sync::LazyLock<JsonrpcMessage> = std::sync::LazyLock::new(|| {
            let content = include_str!($file);

            serde_json::from_str(content).unwrap()
        });
    };
}

load!(INIT, "static/init.json");
load!(NI, "static/ni.json");

async fn basic_server_hello(client_hello: JsonrpcMessage) -> anyhow::Result<JsonrpcMessage> {
    let (tc, ts) = tokio::io::duplex(64);

    tokio::task::spawn(async {
        let mut mcp = McpServer::new(Transport::new(ts));

        mcp.run().await;

        tracing::error!("premature end");
    });

    let mut t = Transport::new(tc);

    t.tx((&*INIT).clone()).await?;

    let Frame::Single(server_hello) = t.rx().await.unwrap()? else {
        panic!("");
    };

    t.tx((&*NI).clone()).await?;

    Ok(server_hello)
}
*/
