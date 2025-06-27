use thiserror::Error;

#[derive(Debug, Error)]
#[error("...")]
pub struct Error {}

pub type Result<T> = std::result::Result<T, Error>;
