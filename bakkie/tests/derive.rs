#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use bakkie::{
    framing::{Frame, Msg, Request, RequestId, Transport},
    proto::V20250618::McpServer,
    provisions::{
        Provisions,
        tools::{ToolError, ToolFuture, ToolInput, ToolOutput},
    },
};
use futures::{SinkExt, stream::StreamExt};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::{io::AsyncWriteExt, sync::mpsc};

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

#[bakkie::tool(name = "count_letters")]
async fn count_letters(needle: char, haystack: String) -> Result<usize, ToolError> {
    Ok(haystack
        .chars()
        .filter(|c| *c == needle)
        .collect::<Vec<_>>()
        .len())
}

#[tokio::test]
async fn test_macro_generates_struct() {
    let params: serde_json::Map<String, serde_json::Value> = serde_json::from_str(
        r#"
        {
        "needle": "a",
        "haystack": "banana"
        }
        "#,
    )
    .unwrap();

    let x = (count_letters().tool_fn)(ToolInput {
        request_id: RequestId::String("1".into()),
        params,
    })
    .await
    .unwrap();

    let f: ToolOutput = x.into_tool_output();

    dbg!(f);
}

#[test]
fn test_macro_generates_struct_old() {
    // Test that the macro generated a struct and function that works
    let args = count_lettersArgs {
        needle: 'a',
        haystack: "banana".to_string(),
    };

    let result = tokio_test::block_on(async { count_letters_impl(args).await }).unwrap();
    assert_eq!(result, 3); // 'a' appears 3 times in "banana"
}

#[bakkie::structured]
#[derive(Default)]
pub struct Location {
    lat: f64,
    lon: f64,
}

#[bakkie::tool(name = "count_letters")]
async fn remember_location(name: String, location: Location) -> Result<usize, ToolError> {
    todo!();
}

#[test]
fn test_macro_generates_struct2() {
    // Test that the macro generated a struct and function that works
    let args = remember_locationArgs {
        name: "".into(),
        location: Location { lat: 0.0, lon: 0.0 },
    };

    let t: ToolFuture = Box::pin(async move {
        match remember_location_impl(args).await {
            Ok(r) => Ok(Box::new(r) as Box<dyn bakkie::provisions::tools::IntoToolOutput>),
            Err(e) => Err(e),
        }
    });
}

#[bakkie::tool]
async fn test_tool_func(param: String) -> Result<String, ToolError> {
    Ok(param)
}

#[tokio::test]
async fn test_naming_convention() {
    // Now test_tool_func() returns a Tool struct
    let tool = test_tool_func();
    assert_eq!(tool.particulars.name, "test_tool_func");

    // The implementation is available as test_tool_func_impl()
    let args = test_tool_funcArgs {
        param: "test".to_string(),
    };
    let result = test_tool_func_impl(args).await.unwrap();
    assert_eq!(result, "test");
}

#[bakkie::tool]
async fn greet(name: String) -> Result<String, ToolError> {
    Ok(format!("Hello, {name}"))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn call_derived_tool() -> anyhow::Result<()> {
    let (mut client, server) = tokio::io::duplex(64);

    tokio::task::spawn(async move {
        let provisions = Provisions::default();
        provisions.insert_tool("greet", greet()).await;

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

    let _tools_response = framed.next().await;

    Ok(())
}
