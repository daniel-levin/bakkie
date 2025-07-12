use bakkie::{
    framing::RequestId,
    provisions::tools::{ToolError, ToolInput},
};
use bakkie_derive::tool;

#[tool]
async fn test_function(message: String, count: u32) -> Result<String, ToolError> {
    Ok(format!("{} (repeated {} times)", message, count))
}

#[tokio::test]
async fn test_generated_tool_fn() {
    // Get the generated tool
    let tool = test_function_tool();

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
    };

    // Call the tool function
    let result = (tool.tool_fn)(tool_input).await;

    // Verify the result
    assert!(result.is_ok());
    let _output = result.unwrap();

    // Just verify we got a successful result - detailed testing would require
    // implementing the ToolOutput properly
}
