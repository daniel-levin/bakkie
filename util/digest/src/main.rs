use std::collections::HashMap;

use bakkie::{
    App, McpServer, Provisions,
    provisions::tools::{Result, ToolError},
};

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
