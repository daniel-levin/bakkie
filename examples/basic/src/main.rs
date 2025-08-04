use bakkie::{
    McpServer, Provisions,
    provisions::tools::{Result, ToolError},
};
use sha2::{Digest, Sha256};

/// Compute the sha256 of a string and return it in hex representation
#[bakkie::tool(title = "__basic__compute_sha2")]
async fn compute_sha256(input: String) -> Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provisions = Provisions::default();
    provisions.insert_tool(compute_sha256).await;

    let server = McpServer::new_with_provisions(bakkie::stdio(), provisions);
    server.run().await?;
    Ok(())
}
