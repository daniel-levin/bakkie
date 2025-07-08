use bakkie_schema::V20250618::JsonrpcMessage;
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Frame {
    Batch(Vec<Msg>),

    Single(Msg),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum RequestId {
    String(String),
    Integer(i64),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Msg {
    Request(Request),
    Notification(Notification),
    Response(Response),
    Error(Error),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Request {
    pub jsonrpc: monostate::MustBe!("2.0"),
    pub method: String,
    pub params: Value,
    pub id: RequestId,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Notification {
    pub jsonrpc: monostate::MustBe!("2.0"),
    pub method: String,

    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Response {
    pub jsonrpc: monostate::MustBe!("2.0"),
    pub result: Value,
    pub id: RequestId,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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
    async fn basic() -> anyhow::Result<()> {
        let input = include_bytes!("../testdata/items.jsonl");

        let frames = include_str!("../testdata/items.jsonl")
            .split("\n")
            .filter(|l| !l.is_empty())
            .map(|l| serde_json::from_str::<Frame>(l).unwrap())
            .collect::<Vec<Frame>>();

        let (mut tc, ts) = tokio::io::duplex(64);

        tokio::task::spawn(async move {
            for b in input {
                let _ = tc.write(&[*b]).await;
            }
        });

        let mut t = Transport::new(ts);

        let mut rx_frames = vec![];

        while let Some(Ok(frame)) = t.rx().await {
            rx_frames.push(frame);
        }

        assert_eq!(frames, rx_frames);

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn malformed_json() -> anyhow::Result<()> {
        let (mut tc, ts) = tokio::io::duplex(64);

        tokio::task::spawn(async move {
            let _ = tc.write("}}".as_bytes()).await;
        });

        let mut t = Transport::new(ts);

        let r = t.rx().await;

        assert!(matches!(r, Some(Err(CodecError::JsonError(_)))));

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn malformed_json_rpc() -> anyhow::Result<()> {
        let (mut tc, ts) = tokio::io::duplex(64);

        tokio::task::spawn(async move {
            let _ = tc
                .write(r#"{"jsonrpc": "1.0", "method": "a", "id": "1", "params": {}}"#.as_bytes())
                .await;
        });

        let mut t = Transport::new(ts);

        let r = t.rx().await;

        dbg!(&r);

        assert!(matches!(r, Some(Err(CodecError::JsonError(_)))));

        Ok(())
    }
}
