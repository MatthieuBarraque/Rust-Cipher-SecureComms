use zeroize::Zeroize;
use rand::rngs::OsRng;
use rand::RngCore;

pub fn get_shared_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

pub fn zeroize_shared_key(key: &mut [u8; 32]) {
    key.zeroize();
}