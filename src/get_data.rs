use crate::inventory::Inventory;
use varint::VarInt;

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

    pub fn deserialize(bytes: &[u8]) -> GetData {
        let count = VarInt::decode(&bytes[0..9]).unwrap();
        let varint_size = VarInt::get_size(count).unwrap();
        let bytes_inventory = &bytes[(varint_size) as usize..];
        let mut inventory: Vec<Inventory> = Vec::new();
        for i in 0..count {
            inventory.push(Inventory::deserialize(
                &bytes_inventory[(i * 36) as usize..((i + 1) * 36) as usize]
                    .try_into()
                    .unwrap(),
            ));
        }
        Self { count, inventory }
    }
}
