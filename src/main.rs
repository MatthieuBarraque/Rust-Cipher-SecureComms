use std::net::TcpListener;
use std::sync::Arc;
use std::env;
use log::{error, info};
use secure_chat::messages;
use secure_chat::server::handlers::start_server;
use secure_chat::security::authentication::Authentication;
use secure_chat::storage::key_manager::KeyManager;

const SERVER_ADDRESS: &str = "127.0.0.1:7878";

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let auth = Arc::new(Authentication::new());
    let key_manager = Arc::new(KeyManager::new("keys/"));

    match TcpListener::bind(SERVER_ADDRESS) {
        Ok(listener) => {
            info!("{}", messages::server_starting(SERVER_ADDRESS));
            info!("{}", messages::server_listening(SERVER_ADDRESS));
            start_server(listener, Arc::clone(&auth), Arc::clone(&key_manager));
        }
        Err(e) => {
            error!("Error binding server: {}", e);
            println!("\x1b[1;33m[INFO] Start client with: cargo run --bin client\x1b[0m");
        }
    }

    if let Some(km) = Arc::get_mut(&mut key_manager.clone()) {
        km.zeroize();
    } else {
        error!("Failed to zeroize key_manager");
    }
}