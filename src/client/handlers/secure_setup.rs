use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::Arc;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use ed25519_dalek::{SigningKey, VerifyingKey, Signer}; // Added Signer trait
use x25519_dalek::PublicKey;
use log::info;

use crate::crypto::secure_channel::SecureChannel;
use crate::security::ecdh::ECDH;
use crate::client::types::AuthMessage;

pub fn setup_secure_connection(
    stream: &mut TcpStream,
    signing_key: &SigningKey,
    verifying_key: &VerifyingKey,
) -> Result<Arc<SecureChannel>, Box<dyn std::error::Error>> {
    // Send authentication message
    send_auth_message(stream, signing_key, verifying_key)?;
    
    // Perform ECDH key exchange
    let shared_secret = perform_key_exchange(stream)?;
    
    // Create secure channel
    Ok(Arc::new(SecureChannel::new_with_secret(&shared_secret)))
}

fn send_auth_message(
    stream: &mut TcpStream,
    signing_key: &SigningKey,
    verifying_key: &VerifyingKey,
) -> Result<(), Box<dyn std::error::Error>> {
    // Use try_sign instead of sign and handle the Result
    let signature = signing_key.try_sign(b"authenticate")?;
    let auth_message = AuthMessage::new(
        verifying_key.to_bytes().to_vec(),
        signature.to_bytes().to_vec(),
    );

    let auth_json = serde_json::to_string(&auth_message)?;
    let auth_b64 = STANDARD.encode(auth_json);
    stream.write_all((auth_b64 + "\n").as_bytes())?;
    info!("Authentication message sent");
    Ok(())
}

fn perform_key_exchange(
    stream: &mut TcpStream,
) -> Result<[u8; 32], Box<dyn std::error::Error>> {
    let ecdh = ECDH::new();
    let public_key = ecdh.get_public_key();

    // Send our public key
    stream.write_all((STANDARD.encode(public_key.as_bytes()) + "\n").as_bytes())?;

    // Receive server's public key
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut server_key = String::new();
    reader.read_line(&mut server_key)?;

    let server_key_bytes = STANDARD.decode(server_key.trim_end())?;
    let server_public_key = PublicKey::from(<[u8; 32]>::try_from(server_key_bytes.as_slice())?);

    Ok(ecdh.derive_shared_secret(&server_public_key))
}