use sha2::{Digest, Sha256};

pub fn double_hash(message: &Vec<u8>) -> [u8; 32] {
    let mut digest = Sha256::digest(message);
    digest = Sha256::digest(digest);

    digest.try_into().unwrap()
}

pub fn checksum(message: &Vec<u8>) -> [u8; 4] {
    let hash = double_hash(message);

    hash[0..4].try_into().unwrap()
}
