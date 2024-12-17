use rand::rngs::OsRng;
use x25519_dalek::{StaticSecret, PublicKey};

pub struct ECDH {
    private_key: StaticSecret,
    public_key: PublicKey,
}

impl ECDH {
    pub fn new() -> Self {
        // Remplacer la fonction dépréciée `StaticSecret::new` par `StaticSecret::random_from_rng`
        let mut rng = OsRng;
        let private_key = StaticSecret::random_from_rng(&mut rng);
        let public_key = PublicKey::from(&private_key);
        ECDH { private_key, public_key }
    }

    pub fn get_public_key(&self) -> PublicKey {
        self.public_key
    }

    pub fn derive_shared_secret(&self, peer_public_key: &PublicKey) -> [u8; 32] {
        self.private_key.diffie_hellman(peer_public_key).to_bytes()
    }
}
