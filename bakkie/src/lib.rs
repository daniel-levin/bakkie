use thiserror::Error;

pub use bakkie_derive::{structured, tool};
use serde_json::{Map, Value};

pub mod framing;
pub mod proto;
pub mod provisions;

pub mod schemars {
    pub use schemars::*;
}

pub mod serde {
    pub use serde::*;
}

use serde::{Deserialize, Serialize};

pub trait Structured: Sized {
    fn as_json_schema() -> serde_json::Value;

    fn from_json(j: Map<String, Value>) -> Result<Self, serde_json::Error>;

    fn as_json_value(&self) -> serde_json::Value;
}

impl<T> Structured for T
where
    T: schemars::JsonSchema + Serialize + for<'de> Deserialize<'de>,
{
    fn as_json_schema() -> serde_json::Value {
        let schema = schemars::schema_for!(T);
        serde_json::to_value(schema).unwrap()
    }

    fn from_json(j: Map<String, Value>) -> Result<Self, serde_json::Error> {
        serde_json::from_value(Value::Object(j))
    }

    fn as_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct BakkieError {
    err: BakkieErrorInternal,
}

impl<T: Into<BakkieErrorInternal>> From<T> for BakkieError {
    fn from(t: T) -> Self {
        Self { err: t.into() }
    }
}

pub type Result<T, E = BakkieError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
enum BakkieErrorInternal {
    #[error(transparent)]
    CodecError(#[from] framing::CodecError),
}
pub fn stdio() -> crate::framing::StdioTransport {
    tokio::io::join(tokio::io::stdin(), tokio::io::stdout())
}

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
