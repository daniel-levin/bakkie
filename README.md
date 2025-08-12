# bakkie

## Simple MCP server framework for Rust.
[<img alt="crates.io" src="https://img.shields.io/crates/v/bakkie.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/bakkie)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-bakkie?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/bakkie)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/daniel-levin/bakkie/test.yml?branch=main&style=for-the-badge" height="20">](https://github.com/daniel-levin/bakkie/actions)

Bakkie is a framework for building Model Context Protocol (MCP) servers in Rust.
MCP is a protocol that enables AI assistants to securely access external resources
and tools through standardized server implementations.

### Basic Usage

```rust
use bakkie::{
    App, McpServer, Provisions,
    provisions::tools::{Result, ToolError},
};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct MyApp {
    invocations: usize,
}

/// Tabulates the characters that appear in the input for accurate counting.
#[bakkie::tool(title = "__count_letters")]
async fn count_letters(input: String) -> Result<HashMap<char, usize>> {
    let mut res = HashMap::new();
    for ch in input.chars() {
        *res.entry(ch).or_insert(0) += 1;
    }
    Ok(res)
}

#[bakkie::structured]
#[derive(PartialEq, Eq)]
enum Progress {
    Forward,
    Backward,
}

/// Inform the user of some progress
#[bakkie::tool(title = "__inform_progress")]
async fn inform_progress(#[app] app: App<MyApp>, progress: Progress) -> Result<usize> {
    let mut c = app.write().await;
    if progress == Progress::Forward {
        c.invocations += 1;
    }
    Ok(c.invocations)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provisions = Provisions::default();
    provisions.insert_tool(count_letters).await;
    provisions.insert_tool(inform_progress).await;

    let server = McpServer::new_with_provisions_and_application(
        bakkie::stdio(),
        provisions,
        MyApp::default(),
    );
    server.run().await?;
    Ok(())
}
```

### Key Features

- **Tool Definition**: Use `#[bakkie::tool]` to define MCP tools with automatic JSON schema generation
- **State Management**: Access shared application state through the `#[app]` parameter
- **Type Safety**: Structured types with `#[bakkie::structured]` for complex parameters
- **Async Support**: Full async/await support for tool implementations
- **Standard I/O**: Built-in stdio transport for MCP communication

License: Unlicense/MIT
