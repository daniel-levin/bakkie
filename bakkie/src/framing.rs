use serde_json::Value;
use tokio_util::codec::{Decoder, Encoder};

use bytes::{Buf, BytesMut};
use serde::{Deserialize, Serialize};
use serde_json::{StreamDeserializer, de::SliceRead};
use thiserror::Error;
use tokio::io::{AsyncRead, AsyncWrite, Join, Stdin, Stdout};

use tokio_util::codec::Framed;

pub trait Transport: AsyncRead + AsyncWrite + Sized + Unpin + Send + Sync + 'static {
    fn into_framed(self) -> Framed<Self, McpFraming> {
        Framed::new(self, McpFraming)
    }
}

impl<T> Transport for T where T: AsyncRead + AsyncWrite + Sized + Unpin + Send + Sync + 'static {}

pub type StdioTransport = Join<Stdin, Stdout>;

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
                tracing::trace!("rx {}", std::str::from_utf8(src).unwrap());
                src.advance(sd.byte_offset());
                Ok(Some(msg))
            }
            Some(Err(e)) => {
                if !e.is_eof() && !e.is_io() {
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
        tracing::trace!("tx {item:#?}");
        dst.extend_from_slice(&serde_json::to_vec(&item)?);
        dst.extend_from_slice(b"\n");
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Frame {
    Batch(Vec<Msg>),

    Single(Msg),
}

impl Frame {
    pub fn into_messages(self) -> Vec<Msg> {
        match self {
            Self::Batch(b) => b,
            Self::Single(m) => vec![m],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum RequestId {
    String(String),
    Integer(i64),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Msg {
    Request(Request),
    Response(Response),
    Error(Error),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Request {
    pub id: Option<RequestId>,
    pub jsonrpc: monostate::MustBe!("2.0"),
    pub method: String,
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
    #![allow(dead_code)]
    use super::*;
    use futures::stream::StreamExt;
    use tokio::io::AsyncWriteExt;

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

        let mut t = ts.into_framed();

        let mut rx_frames = vec![];

        while let Some(Ok(frame)) = t.next().await {
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

        let mut t = ts.into_framed();

        let r = t.next().await;

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

        let mut t = ts.into_framed();

        let r = t.next().await;

        assert!(matches!(r, Some(Err(CodecError::JsonError(_)))));

        Ok(())
    }
}
