use bakkie::{
    framing::{Frame, Msg, Request, RequestId, Response, Transport},
    proto::V20250618::{InboxError, McpServer, McpServerError},
    provisions::{Provisions, tools::SchemaTools},
};
use futures::{SinkExt, stream::StreamExt};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

#[derive(JsonSchema, Serialize, Deserialize)]
struct SearchRequest {
    query: String,
    limit: Option<u32>,
    filters: Vec<String>,
    case_sensitive: bool,
}

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
            id: Some(RequestId::Integer(ping_id as i64)),
            method: "ping".into(),
            params: None,
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
        id: Some(RequestId::Integer(10)),
        method: "something_else".into(),
        params: None,
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
            input_schema: schemars::schema_for!(String),
            output_schema: None,
        };
        let tool = bakkie::provisions::tools::Tool {
            particulars: tool_particulars,
            tool_fn: Box::new(|_| {
                Box::pin(async {
                    todo!();
                })
            }),
        };

        provisions.insert_tool(tool).await;

        // Add second tool with output schema
        let tool_particulars_2 = bakkie::provisions::tools::ToolParticulars {
            name: "calculate".to_string(),
            title: Some("Calculator".to_string()),
            description: Some("Performs basic calculations".to_string()),
            input_schema: schemars::schema_for!(i32),
            output_schema: Some(schemars::schema_for!(f64)),
        };
        let tool_2 = bakkie::provisions::tools::Tool {
            particulars: tool_particulars_2,
            tool_fn: Box::new(|_| {
                Box::pin(async {
                    todo!();
                })
            }),
        };
        provisions.insert_tool(tool_2).await;

        // Add third tool with no title/description
        let tool_particulars_3 = bakkie::provisions::tools::ToolParticulars {
            name: "validate".to_string(),
            title: None,
            description: None,
            input_schema: schemars::schema_for!(bool),
            output_schema: None,
        };
        let tool_3 = bakkie::provisions::tools::Tool {
            particulars: tool_particulars_3,
            tool_fn: Box::new(|_| {
                Box::pin(async {
                    todo!();
                })
            }),
        };
        provisions.insert_tool(tool_3).await;

        // Add fourth tool with complex input schema
        let tool_particulars_4 = bakkie::provisions::tools::ToolParticulars {
            name: "search".to_string(),
            title: Some("Search Tool".to_string()),
            description: Some("Search with complex parameters".to_string()),
            input_schema: schemars::schema_for!(SearchRequest),
            output_schema: Some(schemars::schema_for!(Vec<String>)),
        };
        let tool_4 = bakkie::provisions::tools::Tool {
            particulars: tool_particulars_4,
            tool_fn: Box::new(|_| {
                Box::pin(async {
                    todo!();
                })
            }),
        };
        provisions.insert_tool(tool_4).await;

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

    let st: SchemaTools = serde_json::from_value(response.result)?;

    let tools_list = st.tools;

    assert_eq!(tools_list.len(), 4);

    // Sort tools by name for consistent assertions
    let mut sorted_tools = tools_list;
    sorted_tools.sort_by(|a, b| a.name.cmp(&b.name));

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
