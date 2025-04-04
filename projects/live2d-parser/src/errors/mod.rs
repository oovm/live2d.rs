use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

mod convert;

#[derive(Debug, Serialize, Deserialize)]
pub enum L2Error {
    EncodeError { format: String, message: String },
    OutOfBounds { rest: usize, request: usize },
    UnknownType { type_id: u32 },
    UnknownError {},
}

impl Error for L2Error {}

impl Display for L2Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            L2Error::OutOfBounds { rest, request } => {
                f.write_str(&format!("Out of bounds: rest={}, request={}", rest, request))
            }
            L2Error::EncodeError { format, message } => {
                f.write_str(&format!("Encode error: format={}, message={}", format, message))
            }
            L2Error::UnknownType { type_id } => f.write_str(&format!("Unknown type: type_id={}", type_id)),
            L2Error::UnknownError {} => f.write_str(""),
        }
    }
}
