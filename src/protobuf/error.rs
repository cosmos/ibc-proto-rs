use alloc::format;
use alloc::string::String;
use core::fmt::Display;
use core::num::TryFromIntError;

use super::erased::TryFrom;
use displaydoc::Display;
use prost::{DecodeError, EncodeError};

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            Error::EncodeMessage(e) => Some(e),
            Error::DecodeMessage(e) => Some(e),
            Error::ParseLength(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug, Display)]
pub enum Error {
    /// error converting message type into domain type: `{reason}`
    TryFromProtobuf { reason: String },
    /// error encoding message into buffer
    EncodeMessage(EncodeError),
    /// error decoding buffer into message
    DecodeMessage(DecodeError),
    /// error parsing encoded length
    ParseLength(TryFromIntError),
}

impl Error {
    pub fn try_from<Raw, T, E>(e: E) -> Error
    where
        E: Display,
        T: TryFrom<Raw, Error = E>,
    {
        Error::TryFromProtobuf {
            reason: format!("{}", e),
        }
    }
}
