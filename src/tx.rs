use varint::VarInt;
use crate::error::DeserializeError;
use std::io::{Cursor, Read};

#[derive(Debug, Clone, PartialEq)]
pub struct Tx {
    pub version: u32,
    pub tx_ins: Vec<TxIn>,
    pub tx_outs: Vec<TxOut>,
    pub lock_time: u32,
}

impl Tx {
    pub fn serialize(&self) -> Vec<u8> {
        todo!();
    }

    // We only know the size of the tx after deserializing it. To know when the next tx start we have to return the value
    pub fn deserialize_with_size(bytes: &[u8]) -> Result<(Tx, u64), DeserializeError> {
        let mut cur = Cursor::new(bytes);

        let mut buf = [0u8; 4];
        cur.read_exact(&mut buf)?;
        let version = u32::from_le_bytes(buf);
    
        // Deserialize tx inputs
        let count = VarInt::decode(cur.remaining_slice())?;
        let varint_size = VarInt::get_size(count)? as u64;
        cur.set_position(cur.position() + varint_size);

        dbg!(count);
        
        let mut tx_ins: Vec<TxIn> = vec![];
        for _ in 0..count {
            let (tx_in, size) = TxIn::deserialize_with_size(cur.remaining_slice())?;
            cur.set_position(cur.position() + size);
    
            tx_ins.push(tx_in);
        }

        // Deserialize tx ouputs
        let count = VarInt::decode(cur.remaining_slice())?;
        let varint_size = VarInt::get_size(count)? as u64;
        cur.set_position(cur.position() + varint_size);
        
        let mut tx_outs : Vec<TxOut> = vec![];
        for _ in 0..count {
            let (tx_out, size) = TxOut::deserialize_with_size(cur.remaining_slice())?;
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
        todo!();
    }

    pub fn deserialize_with_size(bytes: &[u8]) -> Result<(TxIn, u64), DeserializeError> {
        let mut cur = Cursor::new(bytes);

        let mut buf = [0u8; 36];
        cur.read_exact(&mut buf)?;
        let previous_output = Outpoint::deserialize(&buf)?;

        let input_size = VarInt::decode(cur.remaining_slice())?;
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
        todo!();
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Outpoint, DeserializeError> {
        let mut cur = Cursor::new(bytes);

        let mut buf = [0u8; 32];
        cur.read_exact(&mut buf)?;
        let previous_hash = buf;

        let mut buf = [0u8; 4];
        cur.read_exact(&mut buf)?;
        let index = u32::from_le_bytes(buf);

        Ok( Self {  
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
        todo!();
    }

    pub fn deserialize_with_size(bytes: &[u8]) -> Result<(TxOut, u64), DeserializeError> {
        let mut cur = Cursor::new(bytes);

        let mut buf = [0u8; 8];
        cur.read_exact(&mut buf)?;
        let value = i64::from_le_bytes(buf);

        let script_size = VarInt::decode(&cur.remaining_slice())?;
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
