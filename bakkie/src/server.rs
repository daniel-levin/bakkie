use crate::{
    Stream,
    codec::{StdioStream, Transport},
};
use thiserror::Error;
use tokio_util::sync::{CancellationToken, DropGuard};

#[derive(Debug)]
pub struct McpServer<T: Stream> {
    transport: Transport<T>,
    ct: CancellationToken,
}

#[derive(Debug, Error)]
#[error("server shut down with error")]
pub struct ListenError {
    #[source]
    cause: RxError,
}

#[derive(Debug, Error)]
enum RxError {}

impl McpServer<StdioStream> {
    pub fn over_stdio() -> Self {
        Self::new(Transport::over_stdio())
    }
}

impl<T: Stream> McpServer<T> {
    pub fn new(transport: Transport<T>) -> Self {
        let ct = CancellationToken::new();
        Self { transport, ct }
    }

    pub async fn run(self) -> Result<(), ListenError> {
        match self
            .ct
            .run_until_cancelled(Self::rx_loop(self.transport))
            .await
        {
            Some(Ok(())) | None => Ok(()),
            Some(Err(rx_loop_error)) => Err(ListenError {
                cause: rx_loop_error,
            }),
        }
    }

    async fn rx_loop(transport: Transport<T>) -> Result<(), RxError> {
        Ok(())
    }
}
