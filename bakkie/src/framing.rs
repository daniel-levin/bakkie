use bakkie_schema::JsonrpcMessage;
use futures::sink::SinkExt;
use serde_json::{Map, Value};
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
            Some(Err(e)) => {
                if !e.is_eof() {
                    Err(e)?
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
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
    pub fn new(t: T) -> Self {
        Self {
            stream: Framed::new(t, McpFraming),
        }
    }

    pub async fn rx(&mut self) -> Option<Result<Frame, CodecError>> {
        self.stream.next().await
    }

    pub async fn tx(&mut self, msg: Msg) -> Result<(), CodecError> {
        self.stream.send(Frame::Single(msg)).await?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Frame {
    Batch(Vec<Msg>),

    Single(Msg),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    String(String),
    Integer(i64),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Msg {
    Request(Request),
    Notification(Notification),
    Response(Response),
    Error(Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub jsonrpc: monostate::MustBe!("2.0"),
    pub method: String,
    pub params: Value,
    pub id: RequestId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub jsonrpc: monostate::MustBe!("2.0"),
    pub method: String,

    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub jsonrpc: monostate::MustBe!("2.0"),
    pub result: Value,
    pub id: RequestId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub jsonrpc: monostate::MustBe!("2.0"),
    pub error: Value,
    pub id: RequestId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::{io::AsyncWriteExt, net::TcpListener};

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

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn roundtrip() -> anyhow::Result<()> {
        let input = include_bytes!("../testdata/items.jsonl");

        let (mut tc, ts) = tokio::io::duplex(512_000);

        tc.write_all(input).await?;

        let mut t = Transport::new(ts);

        let mut frames = vec![];
        for _ in 1..=11 {
            if let Some(rf) = t.rx().await {
                if let Ok(frame) = rf {
                    frames.push(frame);
                } else {
                    anyhow::bail!("{}", frames.len());
                }
            }
        }

        Ok(())
    }
}
