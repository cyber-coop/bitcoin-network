use sha2::{Digest, Sha256};

pub fn checksum(message: &Vec<u8>) -> [u8; 4] {
    let first_hash = Sha256::digest(message);
    let second_hash = Sha256::digest(first_hash);
    second_hash[0..4].try_into().unwrap()
}
