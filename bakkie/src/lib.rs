use bakkie_schema::JsonrpcMessage;
use bytes::{Buf, BytesMut};
use thiserror::Error;
use tokio::{
    io::{AsyncRead, AsyncWrite, Join, Stdin, Stdout, stdin, stdout},
    net::TcpStream,
};
use tokio_util::codec::{Decoder, Encoder};

pub use bakkie_derive::{prompt, tool};
use tokio_util::codec::Framed;

use futures_util::stream::StreamExt;

pub(crate) mod codec;

pub trait Stream: AsyncRead + AsyncWrite + Unpin + Send + Sync + 'static {}

impl<T> Stream for T where T: AsyncRead + AsyncWrite + Unpin + Send + Sync + 'static {}

pub type StdioStream = Join<Stdin, Stdout>;

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

pub type Result<T> = std::result::Result<T, BakkieError>;

#[derive(Debug, Error)]
enum BakkieErrorInternal {
    #[error(transparent)]
    CodecError(#[from] codec::CodecError),
}

#[derive(Debug)]
pub struct Conversation<T: Stream> {
    stream: Framed<T, codec::McpFraming>,
}

impl Conversation<StdioStream> {
    pub fn from_stdio() -> Self {
        Self {
            stream: Framed::new(tokio::io::join(stdin(), stdout()), codec::McpFraming),
        }
    }
}

impl Conversation<TcpStream> {
    pub fn over_tcp(tcp: TcpStream) -> Self {
        Self {
            stream: Framed::new(tcp, codec::McpFraming),
        }
    }
}

impl<T: Stream> Conversation<T> {
    pub async fn run_to_completion(&mut self) -> Result<()> {
        while let Some(msg) = self.stream.next().await {
            let msg = msg?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn my_test() -> anyhow::Result<()> {
        let contents = include_str!("../testdata/basic");
        let stream = TcpListener::bind("127.0.0.1:0").await?;
        let port = stream.local_addr()?.port();

        let sender = tokio::task::spawn(async move {
            let client = TcpStream::connect(&format!("127.0.0.1:{port}")).await?;

            anyhow::Result::<()>::Ok(())
        });

        let (client, _) = stream.accept().await?;

        let mut conv = Conversation::over_tcp(client);

        conv.run_to_completion().await;

        sender.await?;

        Ok(())
    }
}
