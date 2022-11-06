use crate::address::Address;
use varint::VarInt;

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

    pub fn deserialize(bytes: &[u8]) -> Version {
        let varint = VarInt::decode(&bytes[80..89]).unwrap();
        let varint_size = VarInt::get_size(varint).unwrap();
        Self {
            version: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            services: u64::from_le_bytes(bytes[4..12].try_into().unwrap()),
            timestamp: u64::from_le_bytes(bytes[12..20].try_into().unwrap()),
            addr_recv: Address::deserialize(bytes[20..46].try_into().unwrap()),
            addr_trans: Address::deserialize(bytes[46..72].try_into().unwrap()),
            nonce: u64::from_le_bytes(bytes[72..80].try_into().unwrap()),
            user_agent: String::from_utf8(
                bytes[80 + varint_size as usize..80 + varint_size as usize + varint as usize]
                    .to_vec(),
            )
            .unwrap(),
            start_height: u32::from_le_bytes(
                bytes[bytes.len() - 5..bytes.len() - 1].try_into().unwrap(),
            ),
            relay: match bytes[bytes.len() - 1] {
                0 => false,
                1 => true,
                _ => panic!("Invalid bool in u8"),
            },
        }
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
            ]),
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
