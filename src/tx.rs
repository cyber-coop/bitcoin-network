use varint::VarInt;

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
    pub fn deserialize_with_size(bytes: &[u8]) -> (Tx, usize) {
        let mut offset = 0;

        dbg!(hex::encode(&bytes));

        let version = u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap());
        offset += 4;

        // Deserialize tx inputs
        let count = VarInt::decode(&bytes[offset..offset+9]).unwrap();
        let varint_size = VarInt::get_size(count).unwrap();
        offset += varint_size as usize; 

        dbg!(count);

        let mut tx_ins : Vec<TxIn> = vec![];
        for _n in 1..count {
            let (tx_in, size) = TxIn::deserialize_with_size(&bytes[offset..]);
            offset += size;
            dbg!(size);

            tx_ins.push(tx_in);
        }

        // Deserialize tx ouputs
        let count = VarInt::decode(&bytes[offset..offset+9]).unwrap();
        let varint_size = VarInt::get_size(count).unwrap();
        offset += varint_size as usize; 

        dbg!(count);

        let mut tx_outs : Vec<TxOut> = vec![];
        for _n in 1..count {
            let (tx_out, size) = TxOut::deserialize_with_size(&bytes[offset..]);
            offset += size;

            tx_outs.push(tx_out);
        }

        let lock_time = u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap());
        offset += 4;

        (Self {
            version,
            tx_ins,
            tx_outs,
            lock_time,
        }, offset)
    }

    pub fn deserialize(bytes: &[u8]) -> Tx {
        Self::deserialize_with_size(bytes).0
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

    pub fn deserialize_with_size(bytes: &[u8]) -> (TxIn, usize) {
        let mut offset = 0;

        let previous_output = Outpoint::deserialize(&bytes[0..36]);
        offset += 36;

        let size = VarInt::decode(&bytes[offset..offset+9]).unwrap();
        let varint_size = VarInt::get_size(size).unwrap();
        offset += varint_size as usize;

        let signature_script = bytes[offset..offset+(size as usize)].to_vec();
        offset += size as usize;

        let sequence = u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap());
        offset += 4;

        (Self {
            previous_output,
            signature_script,
            sequence,
        }, offset)
    }

    pub fn deserialize(bytes: &[u8]) -> TxIn {
        Self::deserialize_with_size(bytes).0
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

    pub fn deserialize(bytes: &[u8]) -> Outpoint {
        let previous_hash = bytes[0..32].try_into().unwrap();
        let index = u32::from_le_bytes(bytes[32..36].try_into().unwrap());

        Self {
            previous_hash,
            index,
        }
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

    pub fn deserialize_with_size(bytes: &[u8]) -> (TxOut, usize) {
        let mut offset = 0;

        let value = i64::from_le_bytes(bytes[0..8].try_into().unwrap());
        offset += 8;

        let size = VarInt::decode(&bytes[offset..offset+9]).unwrap();
        let varint_size = VarInt::get_size(size).unwrap();
        offset += varint_size as usize;

        let pk_script = bytes[offset..offset+(size as usize)].to_vec();
        offset += size as usize;

        (Self {
            value,
            pk_script,
        }, 0)
    }

    pub fn deserialize(bytes: &[u8]) -> TxOut {
        Self::deserialize_with_size(bytes).0
    }
}