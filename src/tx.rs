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

        let version = u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap());
        offset += 4;

        // Deserialize tx inputs
        let count = VarInt::decode(&bytes[offset..offset+9]).unwrap();
        let varint_size = VarInt::get_size(count).unwrap();
        offset += varint_size as usize; 

        let mut tx_ins : Vec<TxIn> = vec![];
        for n in 0..count-1 {
            let size = VarInt::decode(&bytes[offset..offset+9]).unwrap() as usize;
            let varint_size = VarInt::get_size(count).unwrap();
            offset += varint_size as usize; 

            let tx_in = TxIn::deserialize(&bytes[offset..offset+size]);
            offset += size;

            tx_ins.push(tx_in);
        }

        // Deserialize tx ouputs
        let count = VarInt::decode(&bytes[offset..offset+9]).unwrap();
        let varint_size = VarInt::get_size(count).unwrap();
        offset += varint_size as usize; 

        let mut tx_outs : Vec<TxOut> = vec![];
        for n in 0..count-1 {
            let size = VarInt::decode(&bytes[offset..offset+9]).unwrap() as usize;
            let varint_size = VarInt::get_size(count).unwrap();
            offset += varint_size as usize; 

            let tx_out = TxOut::deserialize(&bytes[offset..offset+size]);
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
pub struct TxIn {}

impl TxIn {
    pub fn serialize(&self) -> Vec<u8> {
        todo!();
    }

    pub fn deserialize(bytes: &[u8]) -> TxIn {
        Self {}
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TxOut {}

impl TxOut {
    pub fn serialize(&self) -> Vec<u8> {
        todo!();
    }

    pub fn deserialize(bytes: &[u8]) -> TxOut {
        Self {}
    }
}