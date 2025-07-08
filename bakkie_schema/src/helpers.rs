use crate::V20250618::{JsonrpcMessage, JsonrpcResponse, RequestId};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResponseSerializeError {
    #[error("input not convertible to json object")]
    NotAnObject,

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}

pub fn new_response<T: Serialize + ?Sized>(
    id: RequestId,
    body: &T,
) -> Result<JsonrpcMessage, ResponseSerializeError> {
    let serde_json::Value::Object(body) = serde_json::to_value(body)? else {
        return Err(ResponseSerializeError::NotAnObject);
    };

    Ok(JsonrpcMessage::Response(JsonrpcResponse {
        id,
        jsonrpc: "2.0".into(),
        result: crate::V20250618::Result {
            meta: Default::default(),
            extra: body,
        },
    }))
}
