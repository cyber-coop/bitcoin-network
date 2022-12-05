use crate::inventory::Inventory;
use varint::VarInt;
use crate::error::DeserializeError;
use std::io::{Cursor, Read};

#[derive(Debug, Clone, PartialEq)]
pub struct GetData {
    pub count: u64,
    pub inventory: Vec<Inventory>,
}

impl GetData {
    pub fn new(inventory: Vec<Inventory>) -> Self {
        Self {
            count: inventory.len() as u64,
            inventory,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend(VarInt::encode(self.count).unwrap());
        for element in &self.inventory {
            result.extend(element.serialize());
        }
        result
    }

    pub fn deserialize(bytes: &[u8]) -> Result<GetData, DeserializeError> {
        let mut cur = Cursor::new(bytes);

        let count = VarInt::decode(&cur.remaining_slice())?;
        let varint_size = VarInt::get_size(count)? as u64;
        cur.set_position(cur.position() + varint_size);

        let mut inventory: Vec<Inventory> = Vec::new();
        for _i in 0..count {
            let mut buf = [0u8; 36];
            cur.read_exact(&mut buf)?;
            let inv = Inventory::deserialize(&buf)?;

            inventory.push(inv);
        }
        Ok(Self::new(inventory))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_data_serialize() {
        let mut hash =
            hex::decode("5bf400bf44ac7a7cb0542ee7e3f9374f68be2dfdf0d64a654c2def6288b3936b")
                .unwrap();
        hash.reverse();
        assert_eq!(
            GetData::new(vec![Inventory {
                identifier: 1,
                hash: hash.try_into().unwrap(),
            }])
            .serialize(),
            [
                1, 1, 0, 0, 0, 107, 147, 179, 136, 98, 239, 45, 76, 101, 74, 214, 240, 253, 45,
                190, 104, 79, 55, 249, 227, 231, 46, 84, 176, 124, 122, 172, 68, 191, 0, 244, 91,
            ]
        )
    }

    #[test]
    fn test_get_data_deserialize() {
        let mut hash =
            hex::decode("5bf400bf44ac7a7cb0542ee7e3f9374f68be2dfdf0d64a654c2def6288b3936b")
                .unwrap();
        hash.reverse();
        assert_eq!(
            GetData::deserialize(&[
                1, 1, 0, 0, 0, 107, 147, 179, 136, 98, 239, 45, 76, 101, 74, 214, 240, 253, 45,
                190, 104, 79, 55, 249, 227, 231, 46, 84, 176, 124, 122, 172, 68, 191, 0, 244, 91,
            ]).unwrap(),
            GetData::new(vec![Inventory {
                identifier: 1,
                hash: hash.try_into().unwrap(),
            }])
        )
    }
}
