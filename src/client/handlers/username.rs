use std::io::{self, Write};  // Remove BufRead if unused
use std::net::TcpStream;
use std::sync::Arc;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
// Remove log::error if unused

use crate::crypto::secure_channel::SecureChannel;

pub fn handle_username_exchange(
    cipher: &Arc<SecureChannel>,
    stream: &mut TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let _reader = io::BufReader::new(stream.try_clone()?);  // Add underscore if unused
    let mut username = String::new();
    
    io::stdin().read_line(&mut username)?;
    
    let username_enc = cipher.encrypt_message(username.trim());
    let username_b64 = STANDARD.encode(&username_enc);
    stream.write_all((username_b64 + "\n").as_bytes())?;
    
    Ok(())
}