use crate::error::DeserializeError;
use std::io::{Cursor, Read};

#[derive(Debug, Clone, PartialEq)]
pub struct Address {
    pub services: u64,
    pub ip: [u8; 16],
    pub port: u16,
}

impl Address {
    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        result.extend_from_slice(&self.services.to_le_bytes());
        result.extend_from_slice(&self.ip);
        result.extend_from_slice(&self.port.to_le_bytes());

        result
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Address, DeserializeError> {
        let mut cur = Cursor::new(bytes);

        let mut buf = [0u8; 8];
        cur.read_exact(&mut buf)?;
        let services = u64::from_le_bytes(buf);

        let mut buf = [0u8; 16];
        cur.read_exact(&mut buf)?;
        let ip = buf;

        let mut buf = [0u8; 2];
        cur.read_exact(&mut buf)?;
        let port = u16::from_le_bytes(buf);

        Ok(Self { services, ip, port })
    }
}
