use std::sync::{Arc, Mutex};
use std::io::Write;
use crate::crypto::secure_channel::SecureChannel;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

pub fn send_encrypted(stream: &Arc<Mutex<std::net::TcpStream>>, cipher: &SecureChannel, plaintext: &str) {
    let msg_enc = cipher.encrypt_message(plaintext);
    let msg_b64 = STANDARD.encode(&msg_enc);
    let mut s = stream.lock().unwrap();
    let _ = writeln!(s, "{}", msg_b64);
}