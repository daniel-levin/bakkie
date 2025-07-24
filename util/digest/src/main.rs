use bakkie::{
    proto::V20250618::McpServer,
    provisions::{
        Provisions,
        tools::{Result, ToolError},
    },
};

#[bakkie::tool(description = "greet people", title = "greeting tool")]
async fn greet(name: String) -> Result<String> {
    tracing::debug!("greet tool called with name: {}", name);
    let result = format!("Hello, {name}");
    tracing::debug!("greet tool returning: {}", result);
    Ok(result)
}

#[bakkie::structured]
enum Gender {
    Male,
    Female,
    Unknown,
}

#[bakkie::structured]
pub struct Person {
    name: String,
    guessed_gender: Gender,
}

#[bakkie::tool(title = "insert a person into the db")]
async fn insert_into_db(person: Person) -> Result<usize> {
    tracing::debug!("{person:#?} inserted into database");
    Ok(0)
}

#[bakkie::tool(
    title = "Record interaction",
    description = "record an interaction between characters in the play"
)]
async fn record_interaction(speaker: Person, listener: Person) -> Result<String> {
    tracing::debug!("{speaker:#?} talking to {listener:#?} inserted into database");
    todo!();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    bakkie::dnp!();

    let provisions = Provisions::default();

    //provisions.insert_tool(greet()).await;
    //provisions.insert_tool(insert_into_db()).await;
    provisions.insert_tool(record_interaction()).await;

    let server = McpServer::new_with_provisions(bakkie::stdio(), provisions);
    server.run().await?;
    Ok(())
}
