use thiserror::Error;

pub use bakkie_derive::{Argument, input};

pub mod framing;
pub mod prompts;
pub mod proto;
pub mod resources;
pub mod tools;

pub mod schemars {
    pub use schemars::*;
}

pub mod serde {
    pub use serde::*;
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
