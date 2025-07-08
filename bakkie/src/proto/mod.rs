use crate::framing::{CodecError, Frame};
use bakkie_schema::V20250618::{InitializeRequestParams, InitializeResult};
use std::str::FromStr;
use strum::{Display, EnumString};
use thiserror::Error;

pub mod V20250618;

pub trait Mcp {
    async fn handshake(&mut self) -> Result<NegotiatedAgreement, HandshakeError>;

    async fn on_rx(&mut self, frame: Option<Result<Frame, CodecError>>) -> Result<(), RxError>;
}

#[derive(Debug, Error)]
pub enum HandshakeError {
    #[error("did not received expected 'initialize' request")]
    ExpectingInitializeRequest,

    #[error("method '{method}' called in handshake when 'initialize' was expected")]
    WrongMethod { method: String },

    #[error("noncompliant handshake received")]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    CannotAllocResponse(#[from] bakkie_schema::ResponseSerializeError),

    #[error(transparent)]
    Codec(#[from] CodecError),

    #[error("did not receive notification")]
    DidNotReceiveNotification,
}

#[derive(Debug, Error)]
pub enum RxError {}

#[derive(Debug, Display, EnumString)]
pub enum Version {
    #[strum(serialize = "2024-11-05")]
    V20241105,

    #[strum(serialize = "2025-03-26")]
    V20250326,

    #[strum(serialize = "2025-06-18")]
    V20250618,
}

#[derive(Debug)]
pub enum ClientVersion {
    Known(Version),

    Unknown(String),
}

#[derive(Debug)]
pub struct NegotiatedAgreement {
    pub client_requested_version: ClientVersion,
    pub server_requested_version: Version,
}

impl NegotiatedAgreement {
    pub fn new(requested_version: &str) -> Self {
        if let Ok(known_version) = Version::from_str(requested_version) {
            Self {
                client_requested_version: ClientVersion::Known(known_version),
                server_requested_version: Version::V20250618,
            }
        } else {
            Self {
                client_requested_version: ClientVersion::Unknown(requested_version.into()),
                server_requested_version: Version::V20250618,
            }
        }
    }
}
