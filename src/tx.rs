use crate::error::DeserializeError;
use crate::utils;
use std::io::{Cursor, Read};
use varint::VarInt;

#[derive(Debug, Clone, PartialEq)]
pub struct Tx {
    pub version: i32,
    pub tx_ins: Vec<TxIn>,
    pub tx_outs: Vec<TxOut>,
    pub lock_time: u32,
}

impl Tx {
    pub fn hash(&self) -> [u8; 32] {
        let tx = &self.serialize();
        utils::double_hash(tx)
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];

        result.extend(self.version.to_le_bytes());
        result.extend(VarInt::encode(self.tx_ins.len() as u64).unwrap());
        self.tx_ins
            .iter()
            .for_each(|txin| result.extend(txin.serialize()));
        result.extend(VarInt::encode(self.tx_outs.len() as u64).unwrap());
        self.tx_outs
            .iter()
            .for_each(|txout| result.extend(txout.serialize()));
        result.extend(self.lock_time.to_le_bytes());

        result
    }

    // We only know the size of the tx after deserializing it. To know when the next tx start we have to return the value
    pub fn deserialize_with_size(bytes: &[u8]) -> Result<(Tx, u64), DeserializeError> {
        let mut cur = Cursor::new(bytes);

        let mut buf = [0u8; 4];
        cur.read_exact(&mut buf)?;
        let version = i32::from_le_bytes(buf);

        // Deserialize tx inputs
        let count = VarInt::decode(cur.split().1)?;
        let varint_size = VarInt::get_size(count)? as u64;
        cur.set_position(cur.position() + varint_size);

        let mut tx_ins: Vec<TxIn> = vec![];
        for _ in 0..count {
            let (tx_in, size) = TxIn::deserialize_with_size(cur.split().1)?;
            cur.set_position(cur.position() + size);

            tx_ins.push(tx_in);
        }

        // Deserialize tx ouputs
        let count = VarInt::decode(cur.split().1)?;
        let varint_size = VarInt::get_size(count)? as u64;
        cur.set_position(cur.position() + varint_size);

        let mut tx_outs: Vec<TxOut> = vec![];
        for _ in 0..count {
            let (tx_out, size) = TxOut::deserialize_with_size(cur.split().1)?;
            cur.set_position(cur.position() + size);

            tx_outs.push(tx_out);
        }

        let mut buf = [0u8; 4];
        cur.read_exact(&mut buf)?;
        let lock_time = u32::from_le_bytes(buf);

        Ok((
            Self {
                version,
                tx_ins,
                tx_outs,
                lock_time,
            },
            cur.position(),
        ))
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Tx, DeserializeError> {
        Ok(Self::deserialize_with_size(bytes)?.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TxIn {
    pub previous_output: Outpoint,
    pub signature_script: Vec<u8>,
    pub sequence: u32,
}

impl TxIn {
    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];

        result.extend(self.previous_output.serialize());
        result.extend(VarInt::encode(self.signature_script.len() as u64).unwrap());
        result.extend(&self.signature_script);
        result.extend(self.sequence.to_le_bytes());

        result
    }

    pub fn deserialize_with_size(bytes: &[u8]) -> Result<(TxIn, u64), DeserializeError> {
        let mut cur = Cursor::new(bytes);

        let mut buf = [0u8; 36];
        cur.read_exact(&mut buf)?;
        let previous_output = Outpoint::deserialize(&buf)?;

        let input_size = VarInt::decode(cur.split().1)?;
        let varint_size = VarInt::get_size(input_size)? as u64;
        cur.set_position(cur.position() + varint_size);

        let mut buf = vec![0; input_size as usize];
        cur.read_exact(&mut buf)?;
        let signature_script = buf;

        let mut buf = [0u8; 4];
        cur.read_exact(&mut buf)?;
        let sequence = u32::from_le_bytes(buf);

        Ok((
            Self {
                previous_output,
                signature_script,
                sequence,
            },
            cur.position(),
        ))
    }

    pub fn deserialize(bytes: &[u8]) -> Result<TxIn, DeserializeError> {
        Ok(Self::deserialize_with_size(bytes)?.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Outpoint {
    pub previous_hash: [u8; 32],
    pub index: u32,
}

impl Outpoint {
    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];

        result.extend(self.previous_hash.to_vec());
        result.extend(self.index.to_le_bytes());

        result
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Outpoint, DeserializeError> {
        let mut cur = Cursor::new(bytes);

        let mut buf = [0u8; 32];
        cur.read_exact(&mut buf)?;
        let previous_hash = buf;

        let mut buf = [0u8; 4];
        cur.read_exact(&mut buf)?;
        let index = u32::from_le_bytes(buf);

        Ok(Self {
            previous_hash,
            index,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TxOut {
    pub value: i64,
    pub pk_script: Vec<u8>,
}

impl TxOut {
    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];

        result.extend(self.value.to_le_bytes());
        result.extend(VarInt::encode(self.pk_script.len() as u64).unwrap());
        result.extend(&self.pk_script);

        result
    }

    pub fn deserialize_with_size(bytes: &[u8]) -> Result<(TxOut, u64), DeserializeError> {
        let mut cur = Cursor::new(bytes);

        let mut buf = [0u8; 8];
        cur.read_exact(&mut buf)?;
        let value = i64::from_le_bytes(buf);

        let script_size = VarInt::decode(cur.split().1)?;
        let varint_size = VarInt::get_size(script_size)? as u64;
        cur.set_position(cur.position() + varint_size);

        let mut buf = vec![0; script_size as usize];
        cur.read_exact(&mut buf)?;
        let pk_script = buf;

        Ok((Self { value, pk_script }, cur.position()))
    }

    pub fn deserialize(bytes: &[u8]) -> Result<TxOut, DeserializeError> {
        Ok(Self::deserialize_with_size(bytes)?.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_tx() {
        let raw_tx = hex::decode("01000000016277237f8fc506329d1f41c2e9a2bb23647f44460bec2a58a5e3f6f428bb15c2010000006b483045022100d7590246176a68adabb3de7c1a74058db0e39aba905bf7feaa4e8b6a2d5fe2bd0220082385abcfa0e94110445b4578f606eedd7daffd27f387bd98833ed867355d3601210245d41687cf6d72ac6c7e0e4e38043429724aed2fd3bb5a6c6b63f1dcab75f23d0000000002005a6202000000001976a914c664d0aa46ba90d12e79729a2da7e7adfbb6a87588acb81e490c000000001976a914bf2d46e52a44c123cff6ea866eb448249cad17c388ac00000000").unwrap();

        let tx = Tx::deserialize(&raw_tx).unwrap();

        let raw_tx_bis = tx.serialize();

        assert_eq!(raw_tx_bis, raw_tx);
    }
}
