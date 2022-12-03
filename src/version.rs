use crate::address::Address;
use varint::VarInt;
use crate::error::DeserializeError;

#[derive(Debug, Clone, PartialEq)]
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
        result
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, DeserializeError> {
        let mut iter = bytes.iter().cloned();

        let version = u32::from_le_bytes(iter.next_chunk::<4>()?);
        let services = u64::from_le_bytes(iter.next_chunk::<8>()?);
        let timestamp = u64::from_le_bytes(iter.next_chunk::<8>()?);
        let addr_recv = Address::deserialize(&iter.next_chunk::<26>()?);
        let addr_trans = Address::deserialize(&iter.next_chunk::<26>()?);
        let nonce = u64::from_le_bytes(iter.next_chunk::<8>()?);

        let varint = VarInt::decode(&bytes[80..])?;
        let varint_size = VarInt::get_size(varint)? as usize;
        iter.advance_by(varint_size)?;
        let user_agent = String::from_utf8(iter.clone().take(varint as usize).collect())?;
        iter.advance_by(varint as usize)?;

        let start_height = u32::from_le_bytes(iter.next_chunk::<4>()?);
        let relay = match iter.next().ok_or(DeserializeError("Missing bytes".to_owned()))? {
            0 => false,
            1 => true,
            _ => { return Err(DeserializeError("Failed to deserialize relay value".to_owned())) }
        };

        Ok(Self {
            version,
            services,
            timestamp,
            addr_recv,
            addr_trans,
            nonce,
            user_agent,
            start_height,
            relay,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_serialize() {
        let version = Version {
            version: 70004,
            services: 4,
            timestamp: 1667596120,
            addr_recv: Address {
                services: 1,
                ip: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 127, 0, 0, 1],
                port: 0,
            },
            addr_trans: Address {
                services: 1,
                ip: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 127, 0, 0, 1],
                port: 0,
            },
            nonce: 1,
            user_agent: "ethicnology".to_owned(),
            start_height: 0,
            relay: false,
        };
        assert_eq!(
            version.serialize(),
            [
                116, 17, 1, 0, 4, 0, 0, 0, 0, 0, 0, 0, 88, 127, 101, 99, 0, 0, 0, 0, 1, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 127, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 127, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0,
                0, 0, 0, 11, 101, 116, 104, 105, 99, 110, 111, 108, 111, 103, 121, 0, 0, 0, 0, 0,
            ]
        );
    }

    #[test]
    fn test_version_deserialize() {
        assert_eq!(
            Version::deserialize(&[
                116, 17, 1, 0, 4, 0, 0, 0, 0, 0, 0, 0, 88, 127, 101, 99, 0, 0, 0, 0, 1, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 127, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 127, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0,
                0, 0, 0, 11, 101, 116, 104, 105, 99, 110, 111, 108, 111, 103, 121, 0, 0, 0, 0, 0,
            ]).unwrap(),
            Version {
                version: 70004,
                services: 4,
                timestamp: 1667596120,
                addr_recv: Address {
                    services: 1,
                    ip: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 127, 0, 0, 1],
                    port: 0,
                },
                addr_trans: Address {
                    services: 1,
                    ip: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 127, 0, 0, 1],
                    port: 0,
                },
                nonce: 1,
                user_agent: "ethicnology".to_owned(),
                start_height: 0,
                relay: false,
            }
        );
    }
}
