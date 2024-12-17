pub const SERVER_ADDRESS: &str = "127.0.0.1:7878";

pub const BANNER: &str = r#"
╔═══════════════════════════════════════════════════════════════╗
║                   SECURE MESSAGING - v0.1.0                    ║
║                 End-to-End Encrypted Chat                      ║
╚═══════════════════════════════════════════════════════════════╝
"#;

pub const HELP_MESSAGE: &str = r#"
Available commands:
/help   - Show this help message
/list   - List all connected users
/clear  - Clear the screen
/quit   - Exit the chat
/info   - Show encryption details
/dm @user <message> - Send private message
"#;