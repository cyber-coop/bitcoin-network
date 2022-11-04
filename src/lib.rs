mod utils;
use crate::utils::{checksum, slice_to_array};
use varint::VarInt;

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub magic_bytes: [u8; 4],
    pub command: String,
    pub size: u32,
    pub checksum: [u8; 4],
    pub payload: Vec<u8>,
}

impl Message {
    pub fn new(magic_bytes: [u8; 4], command: String, payload: Vec<u8>) -> Self {
        Self {
            magic_bytes,
            command,
            size: payload.len() as u32,
            checksum: checksum(&payload),
            payload,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend(self.magic_bytes);
        let mut command = self.command.as_bytes().to_owned();
        command.resize(12, 0);
        result.extend(command);
        result.extend(self.size.to_le_bytes());
        result.extend(self.checksum);
        result.extend(self.payload.clone());
        return result;
    }

    pub fn deserialize(bytes: Vec<u8>) -> Message {
        let magic_bytes = slice_to_array(&bytes[0..4]);
        let mut command = bytes[4..16].to_vec();
        command.retain(|&x| x != 0);
        let command = String::from_utf8(command).unwrap();
        let payload_size = u32::from_le_bytes(slice_to_array(&bytes[16..20]));
        let payload = bytes[24..24 + (payload_size as usize)].to_vec();
        Self {
            magic_bytes: magic_bytes,
            command: command,
            size: payload_size.clone(),
            checksum: checksum(&payload),
            payload: payload,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Version {
    pub version: u32,
    pub services: u64,
    pub timestamp: u64,
    pub addr_recv: Address,
    pub addr_trans: Address,
    pub nonce: u64,
    pub user_agent: String,
    pub start_height: u32,
    pub relay: bool,
}

impl Version {
    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend(self.version.to_le_bytes());
        result.extend(self.services.to_le_bytes());
        result.extend(self.timestamp.to_le_bytes());
        result.extend(self.addr_recv.serialize());
        result.extend(self.addr_trans.serialize());
        result.extend(self.nonce.to_le_bytes());
        result.extend(VarInt::encode(self.user_agent.len() as u64).unwrap());
        result.extend(self.user_agent.as_bytes());
        result.extend(self.start_height.to_le_bytes());
        result.push(self.relay as u8);
        return result;
    }
}

#[derive(Debug, Clone)]
pub struct Address {
    pub services: u64,
    pub ip: [u8; 16],
    pub port: u16,
}

impl Address {
    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend(self.services.to_le_bytes());
        result.extend(self.ip);
        result.extend(self.port.to_le_bytes());
        return result;
    }
}

pub struct GetData {
    pub count: u64,
    pub inventory: Vec<Inventory>,
}

impl GetData {
    pub fn new(inventory: Vec<Inventory>) -> Self {
        Self {
            count: inventory.len() as u64,
            inventory: inventory,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend(VarInt::encode(self.count).unwrap());
        for element in &self.inventory {
            result.extend(element.serialize());
        }
        return result;
    }
}

pub struct Inventory {
    pub identifier: u32,
    pub hash: [u8; 32],
}
impl Inventory {
    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend(self.identifier.to_le_bytes());
        result.extend(self.hash);
        return result;
    }
}
