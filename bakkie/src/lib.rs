use thiserror::Error;
use tokio::io::Join;
use tokio::io::{AsyncRead, AsyncWrite, Stdin, Stdout, stdin, stdout};

#[derive(Debug, Error)]
#[error("...")]
pub struct Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub use bakkie_derive::prompt;
pub use bakkie_derive::tool;

pub trait Stream: AsyncRead + AsyncWrite + Send + Sync + 'static {}

impl<T> Stream for T where T: AsyncRead + AsyncWrite + Send + Sync + 'static {}

pub type StdioStream = Join<Stdin, Stdout>;

#[derive(Debug)]
pub struct Conversation<T: Stream> {
    stream: T,
}

impl Conversation<StdioStream> {
    pub fn from_stdio() -> Self {
        Self {
            stream: tokio::io::join(stdin(), stdout()),
        }
    }
}
