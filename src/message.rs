use crate::utils::checksum;
use crate::error::DeserializeError;

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
        result
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Message, DeserializeError> {
        let mut iter = bytes.iter().cloned();

        let magic_bytes = iter.next_chunk::<4>()?;
        let mut command = iter.next_chunk::<12>()?.to_vec();
        command.retain(|&x| x != 0);
        let command = String::from_utf8(command)?;
        let size = u32::from_le_bytes(iter.next_chunk::<4>()?);
        let payload = iter.collect::<Vec<u8>>();
        Ok( Self {
            magic_bytes,
            command,
            size,
            checksum: checksum(&payload),
            payload,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialize() {
        let verack = Message::new([0xFC, 0xC1, 0xB7, 0xDC], "verack".to_string(), vec![]);
        assert_eq!(
            verack.serialize(),
            [
                252, 193, 183, 220, 118, 101, 114, 97, 99, 107, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 93,
                246, 224, 226,
            ]
        );
    }

    #[test]
    fn test_message_deserialize() {
        let bytes: [u8; 24] = [
            252, 193, 183, 220, 118, 101, 114, 97, 99, 107, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 93, 246,
            224, 226,
        ];
        assert_eq!(
            Message::deserialize(&bytes).unwrap(),
            Message::new([0xFC, 0xC1, 0xB7, 0xDC], "verack".to_string(), vec![])
        );
    }
}
