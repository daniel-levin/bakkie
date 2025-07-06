use thiserror::Error;

use tokio::io::{AsyncRead, AsyncWrite};

pub use bakkie_derive::{Argument, input, prompt, tool};
use bakkie_schema::ElicitRequestParams;

pub mod framing;
pub(crate) mod proto;
pub(crate) mod server;
pub mod tool;

pub use server::McpServer;

pub mod schemars {
    pub use schemars::*;
}

pub mod serde {
    pub use serde::*;
}

pub trait Stream: AsyncRead + AsyncWrite + Unpin + Send + Sync + 'static {}

impl<T> Stream for T where T: AsyncRead + AsyncWrite + Unpin + Send + Sync + 'static {}

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

#[derive(Debug)]
pub struct App<T: Clone> {
    pub app: T,
}

impl<T: Clone> App<T> {
    pub async fn elicit(&self, r: ElicitRequestParams) -> Result<()> {
        Ok(())
    }
}
