use bakkie::{
    proto::V20250618::McpServer,
    provisions::{Provisions, tools::ToolError},
};

#[bakkie::tool(description = "greet people", title = "greeting tool")]
async fn greet(name: String) -> Result<String, ToolError> {
    tracing::debug!("greet tool called with name: {}", name);
    let result = format!("Hello, {name}");
    tracing::debug!("greet tool returning: {}", result);
    Ok(result)
}

#[bakkie::structured]
#[derive(Debug)]
enum Gender {
    Male,
    Female,
    Unknown,
}

#[bakkie::structured]
#[derive(Debug)]
pub struct Person {
    name: String,
    guessed_gender: Gender,
}

#[bakkie::tool(title = "insert a person into the db")]
async fn insert_into_db(person: Person) -> Result<(), ToolError> {
    tracing::debug!("{person:#?} inserted into database");
    Ok(())
}

#[bakkie::tool(
    title = "Record interaction",
    description = "record an interaction between characters in the play"
)]
async fn record_interaction(speaker: Person, listener: Person) -> Result<(), ToolError> {
    tracing::debug!("{speaker:#?} talking to {listener:#?} inserted into database");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    bakkie::dnp!();

    let provisions = Provisions::default();

    provisions.insert_tool(greet()).await;
    provisions.insert_tool(insert_into_db()).await;
    provisions.insert_tool(record_interaction()).await;

    let server = McpServer::new_with_provisions(bakkie::stdio(), provisions);
    server.run().await?;
    Ok(())
}
