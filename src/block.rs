use crate::tx::{Tx, TxIn, TxOut};
use crate::utils;
use varint::VarInt;
use crate::error::DeserializeError;
use std::io::{Cursor, Read};

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub version: u32,
    // auxpow header (to be compatible with Namecoin and Dogecoin)
    pub auxpow_header: Option<AuxPoWHeader>,
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

        /*self.transactions
            .iter()
            .for_each(|t| {
                result.extend(t.serialize());
            });*/

        result
    }

    pub fn deserialize(bytes: &[u8], auxpow_activated: bool) -> Result<Block, DeserializeError> {
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

        if auxpow_activated && version >= 6422787 {
            let (aux_power, size) = match AuxPoWHeader::deserialize_with_size(&cur.remaining_slice()) {
                Ok((aux_power, size)) => (aux_power, size),
                Err(error) => { return Err(error); },
            };
            cur.set_position(cur.position() + size);
        }


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
            auxpow_header: None,
            previous_hash,
            merkle_root,
            timestamp,
            bits,
            nonce,
            transactions,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuxPoWHeader {
    pub version: u32,
}

impl AuxPoWHeader {
    pub fn deserialize_with_size(bytes: &[u8]) -> Result<(Self, u64), DeserializeError> {
        let mut cur = Cursor::new(bytes);

        let mut buf = [0u8; 4];
        cur.read_exact(&mut buf)?;
        let version = u32::from_le_bytes(buf);

        let count = VarInt::decode(cur.remaining_slice())?;
        let varint_size = VarInt::get_size(count)? as u64;
        cur.set_position(cur.position() + varint_size);

        let mut tx_ins: Vec<TxIn> = vec![];
        for _ in 0..count {
            let (tx_in, size) = TxIn::deserialize_with_size(cur.remaining_slice())?;
            cur.set_position(cur.position() + size);
    
        }

        let count = VarInt::decode(cur.remaining_slice())?;
        let varint_size = VarInt::get_size(count)? as u64;
        cur.set_position(cur.position() + varint_size);
        
        let mut tx_outs : Vec<TxOut> = vec![];
        for _ in 0..count {
            let (tx_out, size) = TxOut::deserialize_with_size(cur.remaining_slice())?;
            cur.set_position(cur.position() + size);
        }

        let mut buf = [0u8; 4];
        cur.read_exact(&mut buf)?;
        let lock_time = u32::from_le_bytes(buf);

        let mut buf = [0u8; 32];
        cur.read_exact(&mut buf)?;
        let parent_hash = buf;

        let count = VarInt::decode(cur.remaining_slice())?;
        let varint_size = VarInt::get_size(count)? as u64;
        cur.set_position(cur.position() + varint_size);

        for _ in 0..count {
            let mut buf = [0u8; 32];
            cur.read_exact(&mut buf)?;
            let merkle_hash = buf;
        }

        let mut buf = [0u8; 4];
        cur.read_exact(&mut buf)?;
        let bitmask = u32::from_le_bytes(buf);

        let count = VarInt::decode(cur.remaining_slice())?;
        let varint_size = VarInt::get_size(count)? as u64;
        cur.set_position(cur.position() + varint_size);

        for _ in 0..count {
            let mut buf = [0u8; 32];
            cur.read_exact(&mut buf)?;
            let merkle_hash = buf;
        }

        let mut buf = [0u8; 4];
        cur.read_exact(&mut buf)?;
        let bitmask = u32::from_le_bytes(buf);

        let mut buf = [0u8; 80];
        cur.read_exact(&mut buf)?;

        Ok((
            Self {
                version,
            },
            cur.position(),
        ))
        
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