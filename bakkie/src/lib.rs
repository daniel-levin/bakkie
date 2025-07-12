use thiserror::Error;

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

pub trait AsJsonSchema {
    fn as_json_schema() -> serde_json::Value;
}

impl<T: schemars::JsonSchema> AsJsonSchema for T {
    fn as_json_schema() -> serde_json::Value {
        let schema = schemars::schema_for!(T);
        serde_json::to_value(schema).unwrap()
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
