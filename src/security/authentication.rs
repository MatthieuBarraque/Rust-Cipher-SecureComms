use ed25519_dalek::{SigningKey, VerifyingKey, Signature as EdSignature, Signer, Verifier};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

pub struct Authentication {
    signing_key: SigningKey,
}

impl Authentication {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        Authentication { signing_key }
    }

    pub fn get_public_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    pub fn sign_message(&self, message: &[u8]) -> EdSignature {
        self.signing_key.sign(message)
    }

    pub fn verify_signature(
        &self,
        public_key: &VerifyingKey,
        message: &[u8],
        signature: &EdSignature,
    ) -> bool {
        public_key.verify(message, signature).is_ok()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthMessage {
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
}

impl AuthMessage {
    pub fn new(public_key: VerifyingKey, signature: EdSignature) -> Self {
        AuthMessage {
            public_key: public_key.to_bytes().to_vec(),
            signature: signature.to_bytes().to_vec(),
        }
    }
}
