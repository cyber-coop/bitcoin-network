#[derive(Debug, Clone, PartialEq)]
pub struct Inventory {
    pub identifier: u32,
    pub hash: [u8; 32],
}
impl Inventory {
    pub fn serialize(&self) -> [u8; 36] {
        let mut result: Vec<u8> = vec![];
        result.extend_from_slice(&self.identifier.to_le_bytes());
        result.extend_from_slice(&self.hash);
        result[..].try_into().unwrap()
    }

    pub fn deserialize(bytes: &[u8; 36]) -> Inventory {
        Self {
            identifier: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            hash: bytes[4..36].try_into().unwrap(),
        }
    }
}
