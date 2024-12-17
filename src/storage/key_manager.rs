use std::collections::HashMap;
use std::sync::Mutex;
use ed25519_dalek::VerifyingKey;
use zeroize::Zeroize;

pub struct KeyManager {
    pub shared_key: [u8; 32],
    user_keys: Mutex<HashMap<String, VerifyingKey>>,
}

impl KeyManager {
    pub fn new(_path: &str) -> Self {
        KeyManager {
            shared_key: [0u8; 32],
            user_keys: Mutex::new(HashMap::new()),
        }
    }

    pub fn set_shared_key(&mut self, key: [u8; 32]) {
        self.shared_key = key;
    }

    pub fn zeroize(&mut self) {
        self.shared_key.zeroize();
    }

    pub fn add_user_key(&self, username: &str, public_key: VerifyingKey) -> bool {
        let mut keys = self.user_keys.lock().unwrap();
        if keys.contains_key(username) {
            false
        } else {
            keys.insert(username.to_string(), public_key);
            true
        }
    }

    pub fn get_user_key(&self, username: &str) -> Option<VerifyingKey> {
        let keys = self.user_keys.lock().unwrap();
        keys.get(username).cloned()
    }
}