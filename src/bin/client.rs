use std::io::{self, Write};
use std::net::TcpStream;
use std::sync::Arc;

use colored::*;
use log::{error, info};

use secure_chat::client::{
    auth::generate_keys,
    config::{BANNER, SERVER_ADDRESS},
    handlers::{
        setup_secure_connection,
        message_handler::MessageHandler,
        username::handle_username_exchange
    },
    ui::display::show_connection_status
};

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buf, record| {
            writeln!(
                buf,
                "{}",
                record.args().to_string().bright_black()
            )
        })
        .init();

    println!("{}", BANNER.bright_cyan());
    println!("{}", "Initializing secure connection...".bright_yellow());

    let (signing_key, verifying_key) = generate_keys();
    let mut stream = match TcpStream::connect(SERVER_ADDRESS) {
        Ok(stream) => stream,
        Err(e) => {
            error!("Failed to connect: {}", e);
            return;
        }
    };
    info!("Connected to server at {}", SERVER_ADDRESS);

    let cipher = match setup_secure_connection(&mut stream, &signing_key, &verifying_key) {
        Ok(cipher) => cipher,
        Err(e) => {
            error!("Failed to establish secure connection: {}", e);
            return;
        }
    };

    show_connection_status();
    
    print!("\n{}", "Enter your username: ".bright_cyan());
    io::stdout().flush().unwrap();

    let message_handler = MessageHandler::new(Arc::clone(&cipher));
    
    if let Err(e) = handle_username_exchange(&cipher, &mut stream) {
        error!("Username setup failed: {}", e);
        return;
    }
    println!("\n{}", "🔐 Session initialized successfully!".green());
    println!("{}", "Type /help to see available commands".bright_yellow());
    println!("{}", "─".repeat(60).bright_black());

    message_handler.start_chat_session(stream);
}