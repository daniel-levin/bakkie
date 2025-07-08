use crate::framing::CodecError;
use std::str::FromStr;
use strum::{Display, EnumString};
use thiserror::Error;

#[allow(non_snake_case)]
pub mod V20250618;

#[derive(Debug, Error)]
pub enum HandshakeError {
    #[error("did not received expected 'initialize' request")]
    ExpectingInitializeRequest,

    #[error("method '{method}' called in handshake when 'initialize' was expected")]
    WrongMethod { method: String },

    #[error("noncompliant handshake received")]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    Codec(#[from] CodecError),

    #[error("did not receive notification")]
    DidNotReceiveNotification,
}

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
