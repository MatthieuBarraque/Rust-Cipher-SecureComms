use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::net::TcpStream;
use crate::crypto::secure_channel::SecureChannel;

pub struct Message {
    pub timestamp: Instant,
    pub content: String
}

pub struct ClientInfo {
    pub username: String,
    pub stream: Arc<Mutex<TcpStream>>,
    pub addr: SocketAddr,
    pub cipher: Arc<SecureChannel>,
    pub last_messages: Vec<Instant>,
}

impl ClientInfo {
    pub fn new(username: String, stream: Arc<Mutex<TcpStream>>, addr: SocketAddr, cipher: Arc<SecureChannel>) -> Self {
        ClientInfo {
            username,
            stream,
            addr,
            cipher,
            last_messages: Vec::new(),
        }
    }

    pub fn can_send(&mut self) -> bool {
        let now = Instant::now();
        self.last_messages.push(now);
        self.last_messages.retain(|&t| now.duration_since(t) < Duration::from_secs(1));
        self.last_messages.len() <= 5
    }
}