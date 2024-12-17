use std::net::SocketAddr;

pub fn server_starting(address: &str) -> String {
    format!("Démarrage du serveur sur {}...", address)
}

pub fn server_listening(address: &str) -> String {
    format!("Serveur en écoute sur {}", address)
}

pub fn client_joined(username: &str, addr: &SocketAddr) -> String {
    format!("{} ({}) a rejoint le chat.", username, addr)
}

pub fn client_left(username: &str, addr: &SocketAddr) -> String {
    format!("{} ({}) s'est déconnecté.", username, addr)
}

pub fn send_error(addr: &SocketAddr, err: &str) -> String {
    format!("Erreur lors de l'envoi à {} : {}", addr, err)
}

pub fn client_connected(address: &str) -> String {
    format!("Connected to server at {}", address)
}

pub fn already_running() -> &'static str {
    "Un serveur est déjà actif. Tentative de connexion en tant que client..."
}

pub fn prompt_username() -> &'static str {
    "Veuillez entrer votre username: "
}