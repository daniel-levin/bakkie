use bakkie_schema::JsonrpcMessage as JsonRpcMessage;
use bytes::{Buf, BytesMut};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::io::{AsyncRead, AsyncWrite, Join, Stdin, Stdout, stdin, stdout};
use tokio_util::codec::{Decoder, Encoder};

pub use bakkie_derive::prompt;
pub use bakkie_derive::tool;
use tokio_util::codec::Framed;

use futures_util::stream::StreamExt;
pub trait Stream: AsyncRead + AsyncWrite + Unpin + Send + Sync + 'static {}

impl<T> Stream for T where T: AsyncRead + AsyncWrite + Unpin + Send + Sync + 'static {}

pub type StdioStream = Join<Stdin, Stdout>;

#[derive(Debug)]
pub struct Conversation<T: Stream> {
    stream: Framed<T, JsonRpcCodec>,
}

#[derive(Debug)]
pub struct JsonRpcCodec;

impl Decoder for JsonRpcCodec {
    type Item = JsonRpcMessage;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let buf = src.chunk();
        if buf.is_empty() {
            return Ok(None);
        }

        if let Some(newline_pos) = buf.iter().position(|&b| b == b'\n') {
            let line = src.split_to(newline_pos + 1);
            let message: JsonRpcMessage = serde_json::from_slice(&line[..line.len() - 1])?;
            Ok(Some(message))
        } else {
            Ok(None)
        }
    }
}

impl Encoder<JsonRpcMessage> for JsonRpcCodec {
    type Error = anyhow::Error;

    fn encode(&mut self, item: JsonRpcMessage, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let json = serde_json::to_writer(dst.as_mut(), &item)?;
        dst.extend_from_slice(b"\n");
        Ok(())
    }
}

impl<T: Stream> Conversation<T> {
    pub async fn run(&mut self) -> Option<Result<JsonRpcMessage, anyhow::Error>> {
        self.stream.next().await
    }
}

impl Conversation<StdioStream> {
    pub fn from_stdio() -> Self {
        Self {
            stream: Framed::new(tokio::io::join(stdin(), stdout()), JsonRpcCodec),
        }
    }
}
