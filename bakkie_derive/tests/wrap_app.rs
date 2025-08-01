use bakkie::{
    framing::RequestId,
    proto::V20250618::App,
    provisions::tools::{ToolError, ToolInput},
};
use bakkie_derive::tool;

#[allow(dead_code)]
struct S {}

#[tool]
async fn stateful_test(
    #[app] _a: App<S>,
    message: String,
    count: u32,
) -> Result<String, ToolError> {
    Ok(format!("{} (repeated {} times)", message, count))
}

#[tokio::test]
async fn test_generated_tool_fn() {
    let tool = stateful_test();

    // Create test input
    let mut params = serde_json::Map::new();
    params.insert(
        "message".to_string(),
        serde_json::Value::String("hello".to_string()),
    );
    params.insert(
        "count".to_string(),
        serde_json::Value::Number(serde_json::Number::from(3)),
    );

    let tool_input = ToolInput {
        request_id: RequestId::String("test_123".to_string()),
        params,
        app: App::new(S {}),
    };

    // Call the tool function
    let result = (tool.tool_fn)(tool_input).await;

    // Verify the result
    assert!(result.is_ok());
    let _output = result.unwrap();

    // Just verify we got a successful result - detailed testing would require
    // implementing the ToolOutput properly
}
