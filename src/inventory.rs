use crate::error::DeserializeError;
use std::io::{Cursor, Read};

#[derive(Debug, Clone, PartialEq)]
pub struct Inventory {
    pub identifier: u32,
    pub hash: [u8; 32],
}
impl Inventory {
    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        result.extend_from_slice(&self.identifier.to_le_bytes());
        result.extend_from_slice(&self.hash);
        result
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Inventory, DeserializeError> {
        let mut cur = Cursor::new(bytes);

        let mut buf = [0u8; 4];
        cur.read_exact(&mut buf)?;
        let identifier = u32::from_le_bytes(buf);

        let mut buf = [0u8; 32];
        cur.read_exact(&mut buf)?;
        let hash = buf;

        Ok( Self {
            identifier,
            hash,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory_serialize() {
        let mut hash =
            hex::decode("5bf400bf44ac7a7cb0542ee7e3f9374f68be2dfdf0d64a654c2def6288b3936b")
                .unwrap();
        hash.reverse();
        let bytes_reversed_hash = hash.try_into().unwrap();
        assert_eq!(
            Inventory {
                identifier: 1,
                hash: bytes_reversed_hash,
            }
            .serialize(),
            [
                1, 0, 0, 0, 107, 147, 179, 136, 98, 239, 45, 76, 101, 74, 214, 240, 253, 45, 190,
                104, 79, 55, 249, 227, 231, 46, 84, 176, 124, 122, 172, 68, 191, 0, 244, 91,
            ]
        );
    }

    #[test]
    fn test_inventory_deserialize() {
        let mut hash =
            hex::decode("5bf400bf44ac7a7cb0542ee7e3f9374f68be2dfdf0d64a654c2def6288b3936b")
                .unwrap();
        hash.reverse();
        let bytes_reversed_hash = hash.try_into().unwrap();
        assert_eq!(
            Inventory::deserialize(&[
                1, 0, 0, 0, 107, 147, 179, 136, 98, 239, 45, 76, 101, 74, 214, 240, 253, 45, 190,
                104, 79, 55, 249, 227, 231, 46, 84, 176, 124, 122, 172, 68, 191, 0, 244, 91,
            ]).unwrap(),
            Inventory {
                identifier: 1,
                hash: bytes_reversed_hash,
            }
        );
    }
}
