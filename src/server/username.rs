use std::io::BufRead;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use crate::crypto::secure_channel::SecureChannel;
use crate::messages;
use crate::util::send_encrypted::send_encrypted;

pub fn request_username(
    stream: &Arc<Mutex<std::net::TcpStream>>,
    reader: &mut std::io::BufReader<std::net::TcpStream>,
    cipher: &SecureChannel,
    peer_addr: &SocketAddr,
) -> Option<String> {
    let prompt = messages::prompt_username();
    send_encrypted(stream, cipher, prompt);

    let mut buffer = String::new();
    if reader.read_line(&mut buffer).ok()? == 0 {
        return None;
    }
    let decoded = STANDARD.decode(buffer.trim_end()).ok()?;
    let username = cipher.decrypt_message(&decoded)?;

    let username = username.trim().to_string();
    Some(if username.is_empty() {
        format!("User_{}", peer_addr.port())
    } else {
        username
    })
}
