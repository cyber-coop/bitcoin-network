use crate::error::DeserializeError;
use std::io::{Cursor, Read};

#[derive(Debug, Clone, PartialEq)]
pub struct Ping {
    pub nonce: u64,
}

impl Ping {
    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        result.extend_from_slice(&self.nonce.to_le_bytes());
        
        result
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, DeserializeError> {
        let mut cur = Cursor::new(bytes);

        let mut buf = [0u8; 8];
        cur.read_exact(&mut buf)?;
        let nonce = u64::from_le_bytes(buf);

        Ok(Self {
            nonce,
        })
    }
}

pub type Pong = Ping;
