use std::sync::{Arc, Mutex};

use crate::server::client_info::ClientInfo;
use crate::crypto::secure_channel::SecureChannel;
use crate::util::send_encrypted::send_encrypted;

pub fn handle_command(
    message: &str,
    clients: &Arc<Mutex<Vec<ClientInfo>>>,
    cipher: &Arc<SecureChannel>,
    stream: &Arc<Mutex<std::net::TcpStream>>,
) -> bool {
    if message.starts_with('/') {
        let parts: Vec<&str> = message.split_whitespace().collect();
        match parts[0] {
            "/help" => {
                let help_msg = "[INFO] Commandes disponibles: /help, /list, /quit (client)";
                send_encrypted(stream, cipher, help_msg);
            }
            "/list" => {
                let clients_lock = clients.lock().unwrap();
                let user_list: Vec<String> = clients_lock.iter()
                    .map(|c| c.username.clone())
                    .collect();
                let list_msg = format!("[INFO] Utilisateurs connectés: {}", user_list.join(", "));
                send_encrypted(stream, cipher, &list_msg);
            }
            _ => {
                send_encrypted(stream, cipher, "[INFO] Commande inconnue. Tapez /help.");
            }
        }
        true
    } else {
        false
    }
}