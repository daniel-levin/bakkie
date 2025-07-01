use bakkie_schema::JsonrpcMessage;
use bytes::{Buf, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use serde_json::{StreamDeserializer, de::SliceRead};
use thiserror::Error;

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
