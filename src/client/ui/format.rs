use chrono::Local;
use colored::*;

pub fn format_message(username: &str, message: &str) -> String {
    let timestamp = Local::now().format("%H:%M:%S");
    format!(
        "{} {} {} {}",
        timestamp.to_string().bright_black(),
        "[".bright_black(),
        username.bright_cyan(),
        "]".bright_black(),
    ) + " " + message
}