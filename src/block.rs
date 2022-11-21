use crate::tx::Tx;
use varint::VarInt;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub version: u32,
    // auxpow header (to be compatible with Namecoin and Dogecoin)
    // pub auxpow_header: Option<AuxPoWHeader>
    pub previous_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u32,
    pub bits: u32,
    pub nonce: u32,
    pub transactions: Vec<Tx>
}

impl Block {
    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        result.extend(self.version.to_le_bytes());

        result
    }

    pub fn deserialize(bytes: &[u8]) -> Block {

        // Block header
        let version = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let previous_hash = bytes[4..36].try_into().unwrap();
        let merkle_root = bytes[36..68].try_into().unwrap();
        let timestamp = u32::from_le_bytes(bytes[68..72].try_into().unwrap());
        let bits = u32::from_le_bytes(bytes[72..76].try_into().unwrap());
        let nonce = u32::from_le_bytes(bytes[76..80].try_into().unwrap());

        let mut offset = 80;
        let count = VarInt::decode(&bytes[80..89]).unwrap();
        let varint_size = VarInt::get_size(count).unwrap();
        offset += varint_size as usize;

        let mut transactions : Vec<Tx> = vec![];

        for n in 0..count-1 {
            let (tx, size) = Tx::deserialize_with_size(&bytes[offset..]);
            offset += size;

            transactions.push(tx);
        }

        Self {
            version,
            previous_hash,
            merkle_root,
            timestamp,
            bits,
            nonce,
            transactions
        }
    }
}