use bakkie::{
    framing::StdioTransport,
    proto::V20250618::McpServer,
    provisions::{Provisions, tools::ToolError},
};
use std::fs::OpenOptions;
use tokio::io;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[bakkie::tool(description = "greet people", title = "greeting tool")]
async fn greet(name: String) -> Result<String, ToolError> {
    tracing::debug!("greet tool called with name: {}", name);
    let result = format!("Hello, {name}");
    tracing::debug!("greet tool returning: {}", result);
    Ok(result)
}

#[bakkie::structured]
#[derive(Debug)]
pub struct Person {
    name: String,
    surname: String,
}

#[bakkie::tool(title = "insert a person into the db")]
async fn insert_into_db(person: Person) -> Result<(), ToolError> {
    tracing::debug!("{person:#?} inserted into database");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup tracing to write to named pipe
    let pipe_path = "/tmp/digest_trace";
    let pipe_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(pipe_path)?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(pipe_file)
                .with_ansi(false),
        )
        .with(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::TRACE.into()),
        )
        .init();

    tracing::info!("Digest starting up, tracing to {}", pipe_path);

    let provisions = Provisions::default();
    tracing::debug!("Created default provisions");

    provisions.insert_tool(greet()).await;
    provisions.insert_tool(insert_into_db()).await;

    let stdio: StdioTransport = io::join(io::stdin(), io::stdout());
    tracing::debug!("Created stdio transport");

    let server = McpServer::new_with_provisions(stdio, provisions);
    tracing::debug!("Created MCP server");

    tracing::info!("Starting server run loop");
    server.run().await?;
    tracing::info!("Server run loop completed");
    Ok(())
}
