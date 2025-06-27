use thiserror::Error;

pub use bakkie_derive::tool;

#[derive(Debug, Error)]
#[error("...")]
pub struct Error {}

pub type Result<T> = std::result::Result<T, Error>;
