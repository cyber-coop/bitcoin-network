use varint::VarInt;

#[derive(Debug, Clone, PartialEq)]
pub struct GetBlocks {
    pub version: u32,
    pub hash_count: u64,
    pub block_header_hashes: Vec<[u8; 32]>,
    pub stop_hash: Option<[u8; 32]>,
}

impl GetBlocks {
    pub fn new(
        version: u32,
        block_header_hashes: Vec<[u8; 32]>,
        stop_hash: Option<[u8; 32]>,
    ) -> GetBlocks {
        Self {
            version,
            hash_count: block_header_hashes.len() as u64,
            block_header_hashes,
            stop_hash,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend(self.version.to_le_bytes());
        result.extend(VarInt::encode(self.hash_count).unwrap());
        for element in &self.block_header_hashes {
            result.extend(element);
        }
        match self.stop_hash {
            Some(x) => result.extend(x),
            None => result.extend([0_u8; 32]),
        }
        result
    }

    pub fn deserialize(bytes: &[u8]) -> GetBlocks {
        let hash_count = VarInt::decode(&bytes[4..13]).unwrap();
        let varint_size = VarInt::get_size(hash_count).unwrap();
        let bytes_block_header_hashes = &bytes[(varint_size) as usize..];
        let mut block_header_hashes: Vec<[u8; 32]> = Vec::new();
        for i in 0..hash_count {
            block_header_hashes.push(
                bytes_block_header_hashes[(i * 32) as usize..((i + 1) * 32) as usize]
                    .try_into()
                    .unwrap(),
            );
        }
        let last_bytes: [u8; 32] = bytes[(bytes.len() - 32)..].try_into().unwrap();
        let stop_hash: Option<[u8; 32]> = match last_bytes {
            x if x == [0_u8; 32] => None,
            _ => Some(last_bytes),
        };
        Self {
            version: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            hash_count: hash_count,
            block_header_hashes,
            stop_hash,
        }
    }
}
