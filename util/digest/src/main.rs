use bakkie::{App, Argument, McpServer, schemars::JsonSchema};
use bakkie_schema::CallToolResult;
use std::{pin::Pin, sync::Arc};
use tokio::sync::Mutex;

#[bakkie::prompt("thing")]
fn sha256(name: String) {
    todo!();
}

#[derive(Debug)]
struct Thing {
    stuff: Vec<usize>,
}

impl Thing {
    pub fn store(&mut self, num: usize) {
        self.stuff.push(num)
    }
}

type MyApp = Arc<Mutex<Thing>>;

#[derive(Debug, bakkie::Argument)]
struct Payload {}

#[bakkie::tool("sha256")]
async fn find_last_ten_logs(app: App<MyApp>, name: String) -> bakkie::Result<CallToolResult> {
    let a = app.elicit(todo!()).await?;

    app.app.lock().await.store(200);

    todo!();
}

fn y(
    serde_stuff: u32,
    clone_of_app: u32,
) -> Pin<Box<dyn Future<Output = bakkie::Result<CallToolResult>>>> {
    Box::pin(find_last_ten_logs(
        App {
            app: Arc::new(Mutex::new(Thing { stuff: vec![] })),
        },
        "".into(),
    ))
}

#[tokio::main]
async fn main() -> bakkie::Result<()> {
    let server = McpServer::over_stdio();

    let _ = server.run().await;

    Ok(())
}
