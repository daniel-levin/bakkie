use std::collections::HashMap;

use bakkie::{
    proto::V20250618::McpServer,
    provisions::{
        Provisions,
        tools::{Result, ToolError},
    },
};

/// Tabulates the characters that appear in the input for accurate counting.
#[bakkie::tool(title = "__count_letters")]
async fn count_letters(input: String) -> Result<HashMap<char, usize>> {
    let mut res = HashMap::new();

    for ch in input.chars() {
        *res.entry(ch).or_insert(0) += 1;
    }

    Ok(res)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    bakkie::dnp!();

    let provisions = Provisions::default();

    //provisions.insert_tool(greet()).await;
    //provisions.insert_tool(insert_into_db()).await;
    provisions.insert_tool(count_letters()).await;

    let server = McpServer::new_with_provisions(bakkie::stdio(), provisions);
    server.run().await?;
    Ok(())
}
