use crate::tx::Tx;
use crate::utils;
use std::io::Error;
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
    pub transactions: Vec<Tx>,
}

impl Block {
    pub fn hash(&self) -> Result<[u8; 32], Error> {
        let block_header = &self.serialize_header().unwrap();
        Ok(utils::double_hash(block_header))
    }

    pub fn serialize_header(&self) -> Result<Vec<u8>, Error> {
        let mut result: Vec<u8> = vec![];
        result.extend(self.version.to_le_bytes());
        result.extend(self.previous_hash);
        result.extend(self.merkle_root);
        result.extend(self.timestamp.to_le_bytes());
        result.extend(self.bits.to_le_bytes());
        result.extend(self.nonce.to_le_bytes());
        Ok(result)
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut result: Vec<u8> = vec![];
        result.extend(self.version.to_le_bytes());
        result.extend(self.previous_hash);
        result.extend(self.merkle_root);
        result.extend(self.timestamp.to_le_bytes());
        result.extend(self.bits.to_le_bytes());
        result.extend(self.nonce.to_le_bytes());
        //TODO: serialize transactions
        Ok(result)
    }

    pub fn deserialize(bytes: &[u8]) -> Block {
        // Block header
        let version = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let previous_hash = bytes[4..36].try_into().unwrap();
        let merkle_root = bytes[36..68].try_into().unwrap();
        let timestamp = u32::from_le_bytes(bytes[68..72].try_into().unwrap());
        let bits = u32::from_le_bytes(bytes[72..76].try_into().unwrap());
        let nonce = u32::from_le_bytes(bytes[76..80].try_into().unwrap());

        let count = VarInt::decode(&bytes[80..89]).unwrap();
        let varint_size = VarInt::get_size(count).unwrap();

        let mut offset = 80 + varint_size as usize;
        let mut transactions: Vec<Tx> = vec![];
        for _ in 0..count - 1 {
            let (tx, tx_size) = Tx::deserialize_with_size(&bytes[offset..]);
            offset += tx_size;
            transactions.push(tx);
        }

        Self {
            version,
            previous_hash,
            merkle_root,
            timestamp,
            bits,
            nonce,
            transactions,
        }
    }
}
