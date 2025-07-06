use bakkie::{App, McpServer};
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

#[derive(Debug)]
#[bakkie::input]
struct X {
    s: String,
}

fn p() {
    let x = X { s: "hello".into() };
    let u = serde_json::to_string(&x);
}

#[derive(Debug)]
#[bakkie::input]
struct Payload {
    name: String,
    age: usize,
}

#[bakkie::tool("sha256")]
async fn find_last_ten_logs(app: App<MyApp>, p: Payload) -> bakkie::Result<CallToolResult> {
    app.elicit(todo!()).await?;

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
        todo!(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let schema = bakkie::schemars::schema_for!(Payload);
        let x = serde_json::to_string(&schema);

        dbg!(x);
    }
}

#[tokio::main]
async fn main() -> bakkie::Result<()> {
    let file_appender = tracing_appender::rolling::hourly(".", "prefix.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_writer(non_blocking)
        .init();

    let mut server = McpServer::over_stdio();

    let _ = server.run().await;

    Ok(())
}
