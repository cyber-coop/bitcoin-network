use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct DeserializeError(pub String);

impl Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for DeserializeError {}

impl From<std::io::Error> for DeserializeError {
    fn from(_e: std::io::Error) -> Self {
        DeserializeError("Failed to read varint".to_owned())
    }
}

impl From<std::string::FromUtf8Error> for DeserializeError {
    fn from(_e: std::string::FromUtf8Error) -> Self {
        DeserializeError("Failed to convert from utf8".to_owned())
    }
}
