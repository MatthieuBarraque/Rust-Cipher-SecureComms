use std::io::{self, BufRead, Write};
use std::net::TcpStream;
use std::sync::Arc;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use colored::*;
use log::error;

use crate::crypto::secure_channel::SecureChannel;
use crate::client::ui::display;
use crate::client::config::HELP_MESSAGE;

pub struct MessageHandler {
    cipher: Arc<SecureChannel>,
}

impl MessageHandler {
    pub fn new(cipher: Arc<SecureChannel>) -> Self {
        MessageHandler { cipher }
    }

    pub fn start_chat_session(&self, mut stream: TcpStream) {
        let cipher_clone = Arc::clone(&self.cipher);
        let stream_clone = stream.try_clone().expect("Failed to clone stream");

        // Spawn thread for receiving messages
        std::thread::spawn(move || {
            Self::handle_incoming_messages(stream_clone, cipher_clone);
        });

        // Handle outgoing messages
        self.handle_outgoing_messages(&mut stream);
    }

    fn handle_incoming_messages(stream: TcpStream, cipher: Arc<SecureChannel>) {
        let mut reader = io::BufReader::new(stream);
        let mut message = String::new();

        loop {
            message.clear();
            match reader.read_line(&mut message) {
                Ok(0) => break, // Connection closed
                Ok(_) => {
                    if let Ok(decoded) = STANDARD.decode(message.trim_end()) {
                        if let Some(decrypted) = cipher.decrypt_message(&decoded) {
                            println!("{}", decrypted);
                        }
                    }
                }
                Err(e) => {
                    error!("Error reading message: {}", e);
                    break;
                }
            }
        }
    }

    fn handle_outgoing_messages(&self, stream: &mut TcpStream) {
        for line in io::stdin().lock().lines() {
            match line {
                Ok(message) => {
                    match message.as_str() {
                        "/help" => println!("{}", HELP_MESSAGE.bright_yellow()),
                        "/clear" => print!("\x1B[2J\x1B[1;1H"),
                        "/info" => display::show_security_info(&self.cipher),
                        "/quit" => break,
                        _ => {
                            let encrypted = self.cipher.encrypt_message(&message);
                            let encoded = STANDARD.encode(&encrypted);
                            if stream.write_all((encoded + "\n").as_bytes()).is_err() {
                                error!("Failed to send message");
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Error reading input: {}", e);
                    break;
                }
            }
        }
    }
}