use bakkie::{
    framing::{Frame, Msg, Request, RequestId, Response, Transport},
    proto::V20250618::McpServer,
};
use futures::{SinkExt, stream::StreamExt};
use tokio::io::AsyncWriteExt;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancels_if_corrupt_on_wire() -> anyhow::Result<()> {
    let (mut client, server) = tokio::io::duplex(64);

    let anticipating_error = tokio::task::spawn(async move {
        let server = McpServer::new(server);

        server.run().await
    });

    client.write_all(b"non json").await?;

    let e = anticipating_error.await?.unwrap_err();

    assert!(e.is_cancellation());

    Ok(())
}

static INIT: &str = r#"
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2025-03-26",
    "capabilities": {
      "roots": {
        "listChanged": true
      },
      "sampling": {}
    },
    "clientInfo": {
      "name": "ExampleClient",
      "version": "1.0.0"
    }
  }
}
"#;

static INITIALIZED: &str = r#"
{
  "jsonrpc": "2.0",
  "method": "notifications/initialized"
}
"#;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn initialize_flow() -> anyhow::Result<()> {
    let (mut client, server) = tokio::io::duplex(64);

    let jh = tokio::task::spawn(async move {
        let server = McpServer::new(server);

        server.run().await
    });

    client.write_all(INIT.as_bytes()).await?;

    let mut framed = client.into_framed();

    let _ = framed.next().await.unwrap().unwrap();

    let _ = framed
        .send(serde_json::from_str(INITIALIZED).unwrap())
        .await;

    assert!(!jh.is_finished());

    Ok(())
}

static BAD_INIT: &str = r#"
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
}
"#;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn hangs_up_on_bad_init() -> anyhow::Result<()> {
    let (mut client, server) = tokio::io::duplex(64);

    tokio::task::spawn(async move {
        let server = McpServer::new(server);

        server.run().await
    });

    client.write_all(BAD_INIT.as_bytes()).await?;

    let mut framed = client.into_framed();

    assert!(framed.next().await.is_none());

    Ok(())
}

static NON_INIT: &str = r#"
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "not_initialize",
  "params": {
    "protocolVersion": "2025-03-26",
    "capabilities": {
      "roots": {
        "listChanged": true
      },
      "sampling": {}
    },
    "clientInfo": {
      "name": "ExampleClient",
      "version": "1.0.0"
    }
  }
}
"#;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn hangs_up_on_non_init() -> anyhow::Result<()> {
    let (mut client, server) = tokio::io::duplex(64);

    tokio::task::spawn(async move {
        let server = McpServer::new(server);

        server.run().await
    });

    client.write_all(NON_INIT.as_bytes()).await?;

    let mut framed = client.into_framed();

    assert!(framed.next().await.is_none());

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn allows_pings_before_inited() -> anyhow::Result<()> {
    let (mut client, server) = tokio::io::duplex(64);

    let jh = tokio::task::spawn(async move {
        let server = McpServer::new(server);

        server.run().await
    });

    client.write_all(INIT.as_bytes()).await?;

    let mut framed = client.into_framed();

    let server_hello = framed.next().await;
    assert!(server_hello.is_some());

    for ping_id in 2..=10 {
        let ping = Frame::Single(Msg::Request(Request {
            jsonrpc: monostate::MustBe!("2.0"),
            id: RequestId::Integer(ping_id as i64),
            method: "ping".into(),
            params: serde_json::Value::Null,
        }));

        let _ = framed.send(ping).await;
        let pong = framed.next().await.unwrap()?;

        let Frame::Single(Msg::Response(Response {
            id: RequestId::Integer(id),
            ..
        })) = pong
        else {
            panic!("expected pong");
        };

        assert_eq!(ping_id as i64, id);
    }

    // Finally send an actual initialized notification

    let _ = framed
        .send(serde_json::from_str(INITIALIZED).unwrap())
        .await;

    drop(framed);

    let _ = jh.await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn disallows_non_pings_before_inited() -> anyhow::Result<()> {
    /*
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    */

    let (mut client, server) = tokio::io::duplex(64);

    tokio::task::spawn(async move {
        let server = McpServer::new(server);

        server.run().await
    });

    client.write_all(INIT.as_bytes()).await?;

    let mut framed = client.into_framed();

    let server_hello = framed.next().await;
    assert!(server_hello.is_some());

    let ping = Frame::Single(Msg::Request(Request {
        jsonrpc: monostate::MustBe!("2.0"),
        id: RequestId::Integer(10),
        method: "something_else".into(),
        params: serde_json::Value::Null,
    }));

    let _ = framed.send(ping).await;

    let pong_rcv = framed.next().await.unwrap();

    // they hung up on us
    assert!(pong_rcv.is_err());

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn request_tools() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let (mut client, server) = tokio::io::duplex(64);

    tokio::task::spawn(async move {
        let server = McpServer::new(server);

        server.run().await
    });

    client.write_all(INIT.as_bytes()).await?;

    let mut framed = client.into_framed();

    let server_hello = framed.next().await;
    assert!(server_hello.is_some());

    let _ = framed
        .send(serde_json::from_str(INITIALIZED).unwrap())
        .await;

    let ask_for_tools: Request = serde_json::from_str(r#"
    {
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {
            "cursor": "optional-cursor-value"
        }
    }"#).unwrap();

    let _ = framed.send(Frame::Single(Msg::Request(ask_for_tools))).await;

    let _ = framed.next().await;

    Ok(())
}
