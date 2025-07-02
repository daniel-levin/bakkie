use bakkie_schema::JsonrpcMessage;
use futures::sink::SinkExt;
use tokio_util::codec::{Decoder, Encoder};

use bytes::{Buf, BytesMut};
use serde::{Deserialize, Serialize};
use serde_json::{StreamDeserializer, de::SliceRead};
use thiserror::Error;
use tokio::{
    io::{Join, Stdin, Stdout, stdin, stdout},
    net::TcpStream,
};

use tokio_util::codec::Framed;

use crate::{Result, Stream};
use futures_util::stream::StreamExt;

pub type StdioStream = Join<Stdin, Stdout>;

#[derive(Debug, Error)]
pub enum CodecError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Frame {
    Batch(Vec<JsonrpcMessage>),

    Single(JsonrpcMessage),
}

#[derive(Debug)]
pub struct McpFraming;

impl Decoder for McpFraming {
    type Item = Frame;
    type Error = CodecError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut sd = StreamDeserializer::new(SliceRead::new(src));

        match sd.next() {
            Some(Ok(msg)) => {
                src.advance(sd.byte_offset());
                Ok(Some(msg))
            }
            Some(Err(e)) => Err(e)?,
            None => Ok(None),
        }
    }
}

impl Encoder<Frame> for McpFraming {
    type Error = CodecError;

    fn encode(&mut self, item: Frame, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.extend_from_slice(&serde_json::to_vec(&item)?);
        dst.extend_from_slice(b"\n");
        Ok(())
    }
}

#[derive(Debug)]
pub struct Transport<T: Stream> {
    stream: Framed<T, McpFraming>,
}

impl Transport<StdioStream> {
    pub fn over_stdio() -> Self {
        Self {
            stream: Framed::new(tokio::io::join(stdin(), stdout()), McpFraming),
        }
    }
}

impl Transport<TcpStream> {
    pub fn over_tcp(tcp: TcpStream) -> Self {
        Self {
            stream: Framed::new(tcp, McpFraming),
        }
    }
}

impl<T: Stream> Transport<T> {
    pub async fn rx(&mut self) -> Option<Result<Frame, CodecError>> {
        self.stream.next().await
    }

    pub async fn tx(&mut self, msg: JsonrpcMessage) -> Result<(), CodecError> {
        self.stream.send(Frame::Single(msg)).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;

    #[test]
    fn framing() {
        let Ok(Frame::Single(_)) = serde_json::from_str(
            r#"
            {"jsonrpc": "2.0", "method": "subtract", "params": {}, "id": 1}
        "#,
        ) else {
            panic!("must match")
        };

        let Ok(Frame::Batch(_)) = serde_json::from_str(
            r#"
            [{"jsonrpc": "2.0", "method": "subtract", "params": {}, "id": 1},
            {"jsonrpc": "2.0", "method": "subtract", "params": {}, "id": 1}]
        "#,
        ) else {
            panic!("must match")
        };
    }
}
