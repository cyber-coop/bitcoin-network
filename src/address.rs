use crate::error::DeserializeError;

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
        let mut iter = bytes.iter().cloned();

        let services = u64::from_le_bytes(iter.next_chunk::<8>()?);
        let ip = iter.next_chunk::<16>()?;
        let port = u16::from_le_bytes(iter.next_chunk::<2>()?);

        Ok(Self {
            services,
            ip,
            port,
        })
    }
}
