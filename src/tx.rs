use varint::VarInt;
use crate::error::DeserializeError;

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
    pub fn deserialize_with_size(bytes: &[u8]) -> Result<(Tx, usize), DeserializeError> {
        let mut iter = bytes.iter().cloned();

        let version = u32::from_le_bytes(iter.next_chunk::<4>()?);
        // Deserialize tx inputs
        let count = VarInt::decode(&iter.clone().collect::<Vec<u8>>())?;
        let varint_size = VarInt::get_size(count)?;
        iter.advance_by(varint_size as usize)?;

        let mut tx_ins: Vec<TxIn> = vec![];
        for _ in 1..count {
            let (tx_in, size) = TxIn::deserialize_with_size(&iter.clone().collect::<Vec<u8>>())?;
            iter.advance_by(size)?;
            tx_ins.push(tx_in);
        }

        // Deserialize tx ouputs
        let count = VarInt::decode(&iter.clone().collect::<Vec<u8>>())?;
        let varint_size = VarInt::get_size(count)?;
        iter.advance_by(varint_size as usize)?;
        
        let mut tx_outs : Vec<TxOut> = vec![];
        for _n in 0..count {
            let (tx_out, size) = TxOut::deserialize_with_size(&iter.clone().collect::<Vec<u8>>())?;
            iter.advance_by(size)?;
            tx_outs.push(tx_out);
        }

        let lock_time = u32::from_le_bytes(iter.next_chunk::<4>()?);

        let offset = iter.len() - iter.count();

        Ok((
            Self {
                version,
                tx_ins,
                tx_outs,
                lock_time,
            },
            offset,
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

    pub fn deserialize_with_size(bytes: &[u8]) -> Result<(TxIn, usize), DeserializeError> {
        let mut iter = bytes.iter().cloned();

        let previous_output = Outpoint::deserialize(&iter.next_chunk::<36>()?)?;

        let input_size = VarInt::decode(&iter.clone().collect::<Vec<u8>>())?;
        let varint_size = VarInt::get_size(input_size)?;
        iter.advance_by(varint_size as usize)?;

        let signature_script = iter.clone().take(input_size as usize).collect::<Vec<u8>>();
        iter.advance_by(input_size as usize)?;

        let sequence = u32::from_le_bytes(iter.next_chunk::<4>()?);

        let offset = iter.len() - iter.count();

        Ok((
            Self {
                previous_output,
                signature_script,
                sequence,
            },
            offset,
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
        let mut iter = bytes.iter().cloned();

        let previous_hash = iter.next_chunk::<32>()?;
        let index = u32::from_le_bytes(iter.next_chunk::<4>()?);

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

    pub fn deserialize_with_size(bytes: &[u8]) -> Result<(TxOut, usize), DeserializeError> {
        let mut iter = bytes.iter().cloned();

        let value = i64::from_le_bytes(iter.next_chunk::<8>()?);

        let script_size = VarInt::decode(&iter.clone().collect::<Vec<u8>>())?;
        let varint_size = VarInt::get_size(script_size).unwrap();
        iter.advance_by(varint_size as usize)?;

        let pk_script = iter.clone().take(script_size as usize).collect::<Vec<u8>>();
        iter.advance_by(script_size as usize)?;

        let offset = iter.len() - iter.count();

        Ok((Self { value, pk_script }, offset))
    }

    pub fn deserialize(bytes: &[u8]) -> Result<TxOut, DeserializeError> {
        Ok(Self::deserialize_with_size(bytes)?.0)
    }
}
