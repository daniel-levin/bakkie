/// # Simple MCP server framework for Rust.
pub use bakkie_derive::{structured, tool};

pub mod framing;
pub mod proto;
pub mod provisions;

pub mod schemars {
    pub use schemars::*;
}

pub mod serde {
    pub use serde::*;
}

pub trait InnerSchema {
    fn inner_schema(g: &mut schemars::SchemaGenerator) -> schemars::Schema;
}

impl<T: schemars::JsonSchema, E> InnerSchema for Result<T, E> {
    fn inner_schema(g: &mut schemars::SchemaGenerator) -> schemars::Schema {
        T::json_schema(g)
    }
}

pub fn stdio() -> crate::framing::StdioTransport {
    tokio::io::join(tokio::io::stdin(), tokio::io::stdout())
}

pub use proto::V20250618::{App, McpServer};
pub use provisions::Provisions;

#[macro_export]
macro_rules! dnp {
    () => {
        use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
        // Setup tracing to write to named pipe
        let pipe_path = "/tmp/digest_trace";
        let pipe_file = std::fs::OpenOptions::new()
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
    };
}
