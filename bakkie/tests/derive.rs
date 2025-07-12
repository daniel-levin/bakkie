#![allow(unused_imports)]

use bakkie::{
    framing::{Frame, Msg, Request, Transport},
    proto::V20250618::McpServer,
    provisions::{
        Provisions,
        tools::{ToolError, ToolFuture},
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

#[test]
fn test_macro_generates_struct() {
    // Test that the macro generated a struct and function that works
    let args = CountLettersArgs {
        needle: 'a',
        haystack: "banana".to_string(),
    };

    let t: ToolFuture = Box::pin(async move {
        match count_letters(args).await {
            Ok(r) => Ok(Box::new(r) as Box<dyn bakkie::provisions::tools::IntoToolOutput>),
            Err(e) => Err(e),
        }
    });
}

#[test]
fn test_macro_generates_struct_old() {
    // Test that the macro generated a struct and function that works
    let args = CountLettersArgs {
        needle: 'a',
        haystack: "banana".to_string(),
    };

    let result = tokio_test::block_on(async { count_letters(args).await }).unwrap();
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
    let args = RememberLocationArgs {
        name: "".into(),
        location: Location { lat: 0.0, lon: 0.0 },
    };

    let t: ToolFuture = Box::pin(async move {
        match remember_location(args).await {
            Ok(r) => Ok(Box::new(r) as Box<dyn bakkie::provisions::tools::IntoToolOutput>),
            Err(e) => Err(e),
        }
    });
}

/*
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn call_tool() -> anyhow::Result<()> {
    let (mut client, server) = tokio::io::duplex(64);
    let (tx, mut rx) = mpsc::unbounded_channel();

    tokio::task::spawn(async move {
        let provisions = Provisions::default();

        let tool_particulars_4 = bakkie::provisions::tools::ToolParticulars {
            name: "count_letters".to_string(),
            title: Some("Count letters".to_string()),
            description: Some("Count the number of occurrences of the needle in the haystack".to_string()),
            input_schema: schemars::schema_for!(SearchRequest),
            output_schema: Some(schemars::schema_for!(Vec<String>)),
        };
        let tx_for_tool = tx.clone();
        let tool_4 = bakkie::provisions::tools::Tool {
            particulars: tool_particulars_4,
            tool_fn: Box::new(move |tool_input| {
                let tx_for_future = tx_for_tool.clone();
                Box::pin(async move {
                    // Parse the input parameters
                    let search_req: SearchRequest =
                        serde_json::from_value(serde_json::Value::Object(tool_input.params))
                            .unwrap();

                    // Simulate search results
                    let results = vec![
                        format!("Found: {}", search_req.query),
                        "Result 1".to_string(),
                        "Result 2".to_string(),
                    ];

                    // Send completion signal
                    let _ = tx_for_future.send(search_req.query.clone());

                    // Return the results (though the server doesn't yet handle responses)
                    Ok(Box::new(SearchResults(results))
                        as Box<dyn bakkie::provisions::tools::IntoToolOutput>)
                })
            }),
        };
        provisions.insert_tool("search", tool_4).await;

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

    // Now call the search tool
    let call_tool_request: Request = serde_json::from_str(
        r#"
    {
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "search",
            "arguments": {
                "query": "rust programming",
                "limit": 10,
                "filters": ["documentation", "examples"],
                "case_sensitive": false
            }
        }
    }"#,
    )
    .unwrap();

    let _ = framed
        .send(Frame::Single(Msg::Request(call_tool_request)))
        .await;

    // Wait for the tool to complete and verify it was called
    let received_query = rx.recv().await.unwrap();
    assert_eq!(received_query, "rust programming");

    Ok(())
}
*/
