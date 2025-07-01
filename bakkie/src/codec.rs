use bakkie_schema::JsonrpcMessage;
use serde::Deserializer;
use tokio::io::AsyncWriteExt;
use tokio_util::codec::{Decoder, Encoder};

use bytes::{Buf, BytesMut};
use serde::{Deserialize, Serialize};
use serde_json::{StreamDeserializer, de::SliceRead};
use thiserror::Error;
use tokio::{
    io::{Join, Stdin, Stdout, stdin, stdout},
    net::TcpStream,
};

use std::sync::Arc;
use tokio::sync::Mutex;
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
    type Item = JsonrpcMessage;
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

impl Encoder<JsonrpcMessage> for McpFraming {
    type Error = CodecError;

    fn encode(&mut self, item: JsonrpcMessage, dst: &mut BytesMut) -> Result<(), Self::Error> {
        serde_json::to_writer(dst.as_mut(), &item)?;
        dst.extend_from_slice(b"\n");
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Conversation<T: Stream> {
    stream: Arc<Mutex<Framed<T, McpFraming>>>,
}

impl Conversation<StdioStream> {
    pub fn from_stdio() -> Self {
        Self {
            stream: Arc::new(Mutex::new(Framed::new(
                tokio::io::join(stdin(), stdout()),
                McpFraming,
            ))),
        }
    }
}

impl Conversation<TcpStream> {
    pub fn over_tcp(tcp: TcpStream) -> Self {
        Self {
            stream: Arc::new(Mutex::new(Framed::new(tcp, McpFraming))),
        }
    }
}

impl<T: Stream> Conversation<T> {
    pub async fn run_to_completion(&mut self) -> Result<()> {
        while let Some(msg) = self.stream.lock().await.next().await {
            let msg = msg?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;

    #[test]
    fn single_frame() {
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
    async fn my_test() -> anyhow::Result<()> {
        let contents = include_str!("../testdata/basic");
        let stream = TcpListener::bind("127.0.0.1:0").await?;
        let port = stream.local_addr()?.port();

        let sender = tokio::task::spawn(async move {
            let mut client = TcpStream::connect(&format!("127.0.0.1:{port}")).await?;
            client.write(contents.as_bytes()).await?;

            anyhow::Result::<()>::Ok(())
        });

        let (client, _) = stream.accept().await?;

        let mut conv = Conversation::over_tcp(client);

        conv.run_to_completion().await?;

        sender.await?;

        Ok(())
    }
}
