pub use bakkie_derive::tool;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("...")]
pub struct Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub fn run() -> Result<()> {
    Ok(())
}
