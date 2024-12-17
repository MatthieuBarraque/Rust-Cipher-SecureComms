use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::rngs::OsRng;
use rand::RngCore;

pub struct SecureChannel {
    cipher: Aes256Gcm,
}

impl SecureChannel {
    pub fn new(key: &[u8; 32]) -> Self {
        let key = Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        SecureChannel { cipher }
    }

    pub fn new_with_secret(shared_secret: &[u8; 32]) -> Self {
        Self::new(shared_secret)
    }

    pub fn encrypt_message(&self, message: &str) -> Vec<u8> {
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self.cipher.encrypt(nonce, message.as_bytes())
            .expect("Chiffrement échoué");

        [nonce_bytes.to_vec(), ciphertext].concat() // Nonce + Ciphertext
    }

    pub fn decrypt_message(&self, ciphertext: &[u8]) -> Option<String> {
        if ciphertext.len() < 12 {
            return None;
        }

        let (nonce_bytes, actual_ciphertext) = ciphertext.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        self.cipher.decrypt(nonce, actual_ciphertext)
            .ok()
            .and_then(|plaintext| String::from_utf8(plaintext).ok())
    }
}
