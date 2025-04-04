use crate::L2Error;
use std::string::FromUtf8Error;

impl From<FromUtf8Error> for L2Error {
    fn from(e: FromUtf8Error) -> Self {
        L2Error::EncodeError { format: "utf8".to_string(), message: e.to_string() }
    }
}
