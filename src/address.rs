#[derive(Debug, Clone, PartialEq)]
pub struct Address {
    pub services: u64,
    pub ip: [u8; 16],
    pub port: u16,
}

impl Address {
    pub fn serialize(&self) -> [u8; 26] {
        let mut result: Vec<u8> = vec![];
        result.extend_from_slice(&self.services.to_le_bytes());
        result.extend_from_slice(&self.ip);
        result.extend_from_slice(&self.port.to_le_bytes());
        result[..].try_into().unwrap()
    }

    pub fn deserialize(bytes: &[u8; 26]) -> Address {
        Self {
            services: u64::from_le_bytes(bytes[0..8].try_into().unwrap()),
            ip: bytes[8..24].try_into().unwrap(),
            port: u16::from_le_bytes(bytes[24..26].try_into().unwrap()),
        }
    }
}
