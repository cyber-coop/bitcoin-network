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

impl<const N: usize> From<std::array::IntoIter<u8, N>> for DeserializeError {
    fn from(_e: std::array::IntoIter<u8, N>) -> Self { DeserializeError("Failed to read bytes".to_owned()) }
}

impl From<std::io::Error> for DeserializeError {
    fn from(_e: std::io::Error) -> Self { DeserializeError("Failed to read varint".to_owned()) }
}

impl From<usize> for DeserializeError {
    fn from(_e: usize) -> Self { DeserializeError("Failed to advance bytes".to_owned()) }
}

impl From<std::string::FromUtf8Error> for DeserializeError {
    fn from(_e: std::string::FromUtf8Error) -> Self { DeserializeError("Failed to convert from utf8".to_owned()) }
}
