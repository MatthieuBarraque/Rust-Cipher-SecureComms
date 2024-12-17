use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthMessage {
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
}

impl AuthMessage {
    pub fn new(public_key: Vec<u8>, signature: Vec<u8>) -> Self {
        AuthMessage { public_key, signature }
    }
}