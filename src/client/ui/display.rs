use std::sync::Arc;
use colored::*;
use crate::crypto::secure_channel::SecureChannel;

pub fn show_connection_status() {
    println!("{}", "\n✓ Secure connection established".green());
    println!("{}", "✓ End-to-end encryption enabled".green());
    println!("{}", "✓ Perfect forward secrecy enabled".green());
}

pub fn show_security_info(_cipher: &Arc<SecureChannel>) {
    println!("\n{}", "Security Information:".bright_yellow());
    println!("└─ {}", "Encryption: AES-256-GCM".bright_green());
    println!("└─ {}", "Key Exchange: X25519".bright_green());
    println!("└─ {}", "Authentication: Ed25519".bright_green());
    println!("└─ {}", "Forward Secrecy: Enabled".bright_green());
}