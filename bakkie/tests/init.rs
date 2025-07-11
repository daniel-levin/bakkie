use bakkie::{
    framing::{Frame, Msg, Request, RequestId, Response, Transport},
    proto::V20250618::{InboxError, McpServer, McpServerError},
    provisions::Provisions,
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

    let e: McpServerError = anticipating_error.await?.unwrap_err();

    assert!(matches!(e, McpServerError::InboxError(_)));

    Ok(())
}

static INIT: &str = r#"
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2025-06-18",
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

    // Hang up on us.
    drop(framed);

    // We exit gracefully.
    assert!(jh.await?.is_ok());

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

    let jh = tokio::task::spawn(async move {
        let server = McpServer::new(server);

        server.run().await
    });

    client.write_all(BAD_INIT.as_bytes()).await?;

    let e: McpServerError = jh.await?.unwrap_err();

    assert!(matches!(
        e,
        McpServerError::InboxError(InboxError::InitPhase(_))
    ));

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

    let jh = tokio::task::spawn(async move {
        let server = McpServer::new(server);

        server.run().await
    });

    client.write_all(NON_INIT.as_bytes()).await?;

    let e: McpServerError = jh.await?.unwrap_err();

    assert!(matches!(
        e,
        McpServerError::InboxError(InboxError::InitPhase(_))
    ));

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

    let should_be_graceful = jh.await?;

    assert!(matches!(should_be_graceful, Ok(())));

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn disallows_non_pings_before_inited() -> anyhow::Result<()> {
    let (mut client, server) = tokio::io::duplex(64);

    let jh = tokio::task::spawn(async move {
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

    // they see that we hung up on them
    assert!(pong_rcv.is_err());

    // make sure we actually did hang up on them

    let e: McpServerError = jh.await?.unwrap_err();

    assert!(matches!(
        e,
        McpServerError::InboxError(InboxError::InitPhase(_))
    ));

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn request_tools() -> anyhow::Result<()> {
    let (mut client, server) = tokio::io::duplex(64);

    tokio::task::spawn(async move {
        let provisions = Provisions::default();

        // Add a test tool
        let tool_particulars = bakkie::provisions::tools::ToolParticulars {
            name: "test_tool".to_string(),
            title: Some("Test Tool".to_string()),
            description: Some("A simple test tool".to_string()),
            input_schema: schemars::schema_for!(String).into(),
            output_schema: None,
        };
        let tool = bakkie::provisions::tools::Tool {
            particulars: tool_particulars,
        };

        provisions.insert_tool("test_tool", tool).await;

        let server = McpServer::new_with_provisions(server, provisions);

        server.run().await
    });

    client.write_all(INIT.as_bytes()).await?;

    let mut framed = client.into_framed();

    let server_hello = framed.next().await;
    assert!(server_hello.is_some());

    let _ = framed
        .send(serde_json::from_str(INITIALIZED).unwrap())
        .await;

    let ask_for_tools: Request = serde_json::from_str(
        r#"
    {
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {
            "cursor": "optional-cursor-value"
        }
    }"#,
    )
    .unwrap();

    let _ = framed
        .send(Frame::Single(Msg::Request(ask_for_tools)))
        .await;

    let tools = framed.next().await;

    // Destructure the tools response and make assertions
    let Frame::Single(Msg::Response(response)) =
        tools.ok_or_else(|| anyhow::anyhow!("No response received"))??
    else {
        return Err(anyhow::anyhow!(
            "Expected response frame, got different message type"
        ));
    };

    assert_eq!(response.jsonrpc, monostate::MustBe!("2.0"));
    assert_eq!(response.id, RequestId::Integer(2));

    let tools_list: Vec<bakkie_schema::V20250618::Tool> = serde_json::from_value(response.result)?;
    assert_eq!(tools_list.len(), 1);

    let tool = &tools_list[0];
    assert_eq!(tool.name, "test_tool");
    assert_eq!(tool.title, Some("Test Tool".to_string()));
    assert_eq!(tool.description, Some("A simple test tool".to_string()));
    assert_eq!(tool.input_schema.type_, "string");
    assert!(tool.output_schema.is_none());
    assert!(tool.annotations.is_none());

    Ok(())
}

static OLDER_INIT: &str = r#"
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

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn it_is_an_older_code_sir_but_it_checks_out() -> anyhow::Result<()> {
    let (mut client, server) = tokio::io::duplex(64);

    let jh = tokio::task::spawn(async move {
        let server = McpServer::new(server);

        server.run().await
    });

    client.write_all(OLDER_INIT.as_bytes()).await?;

    let mut framed = client.into_framed();

    let _ = framed.next().await.unwrap().unwrap();

    let _ = framed
        .send(serde_json::from_str(INITIALIZED).unwrap())
        .await;

    // Hang up on us.
    drop(framed);

    // We exit gracefully.
    assert!(jh.await?.is_ok());

    Ok(())
}
