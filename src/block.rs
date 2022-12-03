use crate::tx::Tx;
use crate::utils;
use varint::VarInt;
use crate::error::DeserializeError;

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
        let mut iter = bytes.iter().cloned();

        // Block header
        let version = u32::from_le_bytes(iter.next_chunk::<4>()?);
        let previous_hash = iter.next_chunk::<32>()?;
        let merkle_root = iter.next_chunk::<32>()?;
        let timestamp = u32::from_le_bytes(iter.next_chunk::<4>()?);
        let bits = u32::from_le_bytes(iter.next_chunk::<4>()?);
        let nonce = u32::from_le_bytes(iter.next_chunk::<4>()?);

        let count = VarInt::decode(&bytes[80..]?);
        let varint_size = VarInt::get_size(count).unwrap();
        iter.advance_by(varint_size)?;

        let mut transactions: Vec<Tx> = vec![];
        for _ in 0..count - 1 {
            let (tx, tx_size) = Tx::deserialize_with_size(&iter.clone().collect<Vec<u8>>());
            iter.advance_by(tx_size)?;
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
