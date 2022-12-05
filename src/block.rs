use crate::tx::Tx;
use crate::utils;
use varint::VarInt;
use crate::error::DeserializeError;
use std::io::{Cursor, Read};

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
    pub fn hash(&self) -> [u8; 32] {
        let block_header = &self.serialize_header();
        utils::double_hash(block_header)
    }

    pub fn serialize_header(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        result.extend(self.version.to_le_bytes());
        result.extend(self.previous_hash);
        result.extend(self.merkle_root);
        result.extend(self.timestamp.to_le_bytes());
        result.extend(self.bits.to_le_bytes());
        result.extend(self.nonce.to_le_bytes());
        result
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        result.extend(self.version.to_le_bytes());
        result.extend(self.previous_hash);
        result.extend(self.merkle_root);
        result.extend(self.timestamp.to_le_bytes());
        result.extend(self.bits.to_le_bytes());
        result.extend(self.nonce.to_le_bytes());
        //TODO: serialize transactions
        result
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Block, DeserializeError> {
        let mut cur = Cursor::new(bytes);

        // Block headers
        let mut buf = [0u8; 4];
        cur.read_exact(&mut buf)?;
        let version = u32::from_le_bytes(buf);

        let mut buf = [0u8; 32];
        cur.read_exact(&mut buf)?;
        let previous_hash = buf;

        let mut buf = [0u8; 32];
        cur.read_exact(&mut buf)?;
        let merkle_root = buf;

        let mut buf = [0u8; 4];
        cur.read_exact(&mut buf)?;
        let timestamp = u32::from_le_bytes(buf);
    
        let mut buf = [0u8; 4];
        cur.read_exact(&mut buf)?;
        let bits = u32::from_le_bytes(buf);

        let mut buf = [0u8; 4];
        cur.read_exact(&mut buf)?;
        let nonce = u32::from_le_bytes(buf);

        let count = VarInt::decode(&cur.remaining_slice())?;
        let varint_size = VarInt::get_size(count)?;
        cur.set_position(cur.position() + varint_size as u64);

        let mut transactions: Vec<Tx> = vec![];
        for _ in 0..count {
            let (tx, tx_size) = Tx::deserialize_with_size(&cur.remaining_slice())?;
            cur.set_position(cur.position() + tx_size);

            transactions.push(tx);
        }

        Ok(Self {
            version,
            previous_hash,
            merkle_root,
            timestamp,
            bits,
            nonce,
            transactions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_block_deserialize() {
        let f = fs::read("./raw_50057.bin").unwrap();

        let block = Block::deserialize(&f).expect("should deserialize raw block");
    }
}