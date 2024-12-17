use std::io::{BufRead, BufReader, Write};
use std::net::{TcpStream, SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use ed25519_dalek::{VerifyingKey, Signature as EdSignature};
use x25519_dalek::PublicKey as X25519PublicKey;
use log::{info, warn, error};

use crate::crypto::secure_channel::SecureChannel;
use crate::messages;
use crate::security::authentication::{Authentication, AuthMessage};
use crate::security::ecdh::ECDH;
use crate::server::client_info::ClientInfo;
use crate::server::commands::handle_command;
use crate::server::username::request_username;
use crate::storage::key_manager::KeyManager;
use crate::util::send_encrypted::send_encrypted;

pub fn start_server(
    listener: TcpListener,
    auth: Arc<Authentication>,
    key_manager: Arc<KeyManager>,
) {
    let clients = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients = Arc::clone(&clients);
                let auth = Arc::clone(&auth);
                let key_manager = Arc::clone(&key_manager);
                
                std::thread::spawn(move || {
                    handle_client(stream, clients, auth, key_manager);
                });
            }
            Err(e) => {
                error!("Client connection failed: {}", e);
            }
        }
    }
}

pub fn handle_client(
    stream: TcpStream,
    clients: Arc<Mutex<Vec<ClientInfo>>>,
    auth: Arc<Authentication>,
    key_manager: Arc<KeyManager>,
) {
    let peer_addr = match stream.peer_addr() {
        Ok(addr) => addr,
        Err(_) => return,
    };

    let stream = Arc::new(Mutex::new(stream));
    let mut reader = BufReader::new(stream.lock().unwrap().try_clone().unwrap());

    let client_public_key = match authenticate_client(&stream, &mut reader, &auth) {
        Some(pk) => pk,
        None => {
            info!("Authentication failed for {}", peer_addr);
            return;
        }
    };

    let shared_secret = match perform_ecdh(&stream, &mut reader) {
        Some(secret) => secret,
        None => {
            info!("Key exchange failed for {}", peer_addr);
            return;
        }
    };

    // Initialize secure channel with shared secret
    let session_cipher = Arc::new(SecureChannel::new_with_secret(&shared_secret));

    let username = match request_username(&stream, &mut reader, &session_cipher, &peer_addr) {
        Some(u) => u,
        None => return,
    };

    if !key_manager.add_user_key(&username, client_public_key) {
        send_encrypted(&stream, &session_cipher, "[ERROR] Username already taken.");
        return;
    }

    {
        let mut clients_lock = clients.lock().unwrap();
        clients_lock.push(ClientInfo::new(
            username.clone(),
            Arc::clone(&stream),
            peer_addr,
            Arc::clone(&session_cipher)
        ));
    }

    broadcast_message(
        &messages::client_joined(&username, &peer_addr),
        &clients,
        Some(peer_addr)
    );

    let mut buffer = String::new();
    loop {
        match reader.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                let trimmed = buffer.trim_end();
                let decoded = match STANDARD.decode(trimmed) {
                    Ok(d) => d,
                    Err(_) => {
                        send_encrypted(&stream, &session_cipher, "[ERROR] Invalid Base64 message!");
                        continue;
                    }
                };

                let message = match session_cipher.decrypt_message(&decoded) {
                    Some(m) => m,
                    None => {
                        send_encrypted(&stream, &session_cipher, "[ERROR] Failed to decrypt message!");
                        continue;
                    }
                };

                if handle_command(&message, &clients, &session_cipher, &stream) {
                    continue;
                }

                // Anti-spam check
                {
                    let mut clients_lock = clients.lock().unwrap();
                    if let Some(client) = clients_lock.iter_mut().find(|c| c.addr == peer_addr) {
                        if !client.can_send() {
                            send_encrypted(&stream, &session_cipher, "[ERROR] Too many messages, disconnecting.");
                            break;
                        }
                    }
                }

                let display_msg = format!("[{}] {}", username, message.trim());
                info!("{}", &display_msg);
                broadcast_message(&display_msg, &clients, Some(peer_addr));
            }
            Err(e) => {
                error!("Error reading message: {}", e);
                break;
            }
        }
        buffer.clear();
    }

    broadcast_message(
        &messages::client_left(&username, &peer_addr),
        &clients,
        None
    );

    let mut clients_lock = clients.lock().unwrap();
    clients_lock.retain(|c| c.addr != peer_addr);
}

pub fn broadcast_message(
    message: &str,
    clients: &Arc<Mutex<Vec<ClientInfo>>>,
    exclude_addr: Option<SocketAddr>
) {
    let clients_lock = clients.lock().unwrap();
    
    for client in clients_lock.iter() {
        if Some(client.addr) == exclude_addr {
            continue;
        }

        let encrypted = client.cipher.encrypt_message(message);
        let encoded = STANDARD.encode(&encrypted);

        info!("Sending message to {} [{}]", client.addr, client.username);
        let mut stream = client.stream.lock().unwrap();
        if let Err(e) = writeln!(stream, "{}", encoded) {
            warn!("{}", messages::send_error(&client.addr, &e.to_string()));
        }
    }
}

fn authenticate_client(
    _stream: &Arc<Mutex<TcpStream>>,
    reader: &mut BufReader<TcpStream>,
    auth: &Arc<Authentication>,
) -> Option<VerifyingKey> {
    let mut auth_message_enc = String::new();
    if reader.read_line(&mut auth_message_enc).unwrap_or(0) == 0 {
        error!("Failed to read AuthMessage from client");
        return None;
    }

    let decoded = match STANDARD.decode(auth_message_enc.trim_end()) {
        Ok(d) => d,
        Err(e) => {
            error!("Error decoding AuthMessage Base64: {}", e);
            return None;
        }
    };

    let auth_msg: AuthMessage = match serde_json::from_slice(&decoded) {
        Ok(msg) => msg,
        Err(e) => {
            error!("Error deserializing AuthMessage: {}", e);
            return None;
        }
    };

    let client_public_key = match <[u8; 32]>::try_from(auth_msg.public_key.as_slice()) {
        Ok(bytes) => match VerifyingKey::from_bytes(&bytes) {
            Ok(pk) => pk,
            Err(e) => {
                error!("Error converting public key: {}", e);
                return None;
            }
        },
        Err(e) => {
            error!("Error converting bytes to [u8;32]: {}", e);
            return None;
        }
    };
    info!("Client public key received and converted successfully");

    let signature_bytes = match <[u8; 64]>::try_from(auth_msg.signature.as_slice()) {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Error converting signature bytes to [u8;64]: {}", e);
            return None;
        }
    };
    
    let signature = EdSignature::from_bytes(&signature_bytes);
    info!("Client signature received and converted successfully");

    if !auth.verify_signature(&client_public_key, b"authenticate", &signature) {
        error!("Signature verification failed");
        return None;
    }
    info!("Authentication successful");

    Some(client_public_key)
}

fn perform_ecdh(
    stream: &Arc<Mutex<TcpStream>>,
    reader: &mut BufReader<TcpStream>,
) -> Option<[u8; 32]> {
    let ecdh = ECDH::new();
    let server_public_key = ecdh.get_public_key();

    let server_pub_key_bytes = server_public_key.to_bytes();
    let server_pub_key_b64 = STANDARD.encode(&server_pub_key_bytes);
    writeln!(stream.lock().unwrap(), "{}", server_pub_key_b64).ok()?;
    info!("Server ECDH public key sent");

    let mut client_pub_key_enc = String::new();
    if reader.read_line(&mut client_pub_key_enc).unwrap_or(0) == 0 {
        error!("Failed to read client ECDH public key");
        return None;
    }

    let client_pub_key_bytes = match STANDARD.decode(client_pub_key_enc.trim_end()) {
        Ok(d) => d,
        Err(e) => {
            error!("Error decoding client ECDH public key Base64: {}", e);
            return None;
        }
    };

    if client_pub_key_bytes.len() != 32 {
        error!("Invalid client ECDH public key length");
        return None;
    }

    // Fixed pattern matching
    let client_public_key = match <[u8; 32]>::try_from(client_pub_key_bytes.as_slice()) {
        Ok(bytes) => X25519PublicKey::from(bytes),
        Err(e) => {
            error!("Error converting client ECDH public key bytes: {}", e);
            return None;
        }
    };
    info!("Client ECDH public key received and converted successfully");

    Some(ecdh.derive_shared_secret(&client_public_key))
}