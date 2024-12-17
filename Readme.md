# RUST-SigTerm

## Table des Matières

- [Description](#description)
- [Fonctionnalités](#fonctionnalités)
- [Architecture du Projet](#architecture-du-projet)
- [Installation](#installation)
- [Utilisation](#utilisation)
- [Configuration](#configuration)
- [Sécurité](#sécurité)

## Description

**Secure Messaging** est une application de messagerie sécurisée développée en Rust. Elle permet à plusieurs utilisateurs de communiquer de manière privée et sécurisée via un serveur centralisé. Toutes les communications sont chiffrées afin de garantir la confidentialité et l'intégrité des messages échangés. (languages de golement (RUST))

## Fonctionnalités

- **Chiffrement AES-256-GCM** : Toutes les communications sont chiffrées à l'aide de l'algorithme AES-256-GCM pour assurer une sécurité maximale.
- **Gestion Multi-Clients** : Le serveur peut gérer simultanément plusieurs connexions de clients.
- **Commandes Utilisateur** :
  - `/help` : Affiche la liste des commandes disponibles.
  - `/list` : Liste tous les utilisateurs actuellement connectés.
  - `/quit` : Permet de se déconnecter proprement du serveur.
- **Anti-Spam** : Limite le nombre de messages qu'un utilisateur peut envoyer par seconde pour prévenir les abus.
- **Journalisation** : Enregistre les événements importants tels que les connexions, déconnexions, et erreurs.
- **Gestion des Erreurs** : Envoie des messages d'erreur chiffrés aux clients en cas de problèmes de décodage ou de déchiffrement.

## Architecture du Projet

Le projet est structuré de manière modulaire pour faciliter la maintenance et l'extension des fonctionnalités :


```
.
├── Cargo.lock
├── Cargo.toml
├── Readme.md
├── config.toml
├── docker
│   ├── Dockerfile
│   ├── config.toml
│   ├── docs.md
│   └── entrypoint.sh
├── docker-compose.yml
├── docs.md
├── keys
│   └── key_here.txt
└── src
    ├── bin
    │   └── client.rs
    ├── client
    │   ├── auth.rs
    │   ├── config.rs
    │   ├── handlers
    │   │   ├── message_handler.rs
    │   │   ├── mod.rs
    │   │   ├── secure_setup.rs
    │   │   └── username.rs
    │   ├── mod.rs
    │   ├── types
    │   │   ├── auth_message.rs
    │   │   └── mod.rs
    │   └── ui
    │       ├── display.rs
    │       ├── format.rs
    │       └── mod.rs
    ├── crypto
    │   ├── key.rs
    │   ├── mod.rs
    │   └── secure_channel.rs
    ├── lib.rs
    ├── logger.rs
    ├── main.rs
    ├── messages.rs
    ├── security
    │   ├── authentication.rs
    │   ├── ecdh.rs
    │   └── mod.rs
    ├── server
    │   ├── client_info.rs
    │   ├── commands.rs
    │   ├── handlers.rs
    │   ├── mod.rs
    │   └── username.rs
    ├── storage
    │   ├── key_manager.rs
    │   └── mod.rs
    └── util
        ├── mod.rs
        └── send_encrypted.rs

13 directories, 43 files

```

### Description des Modules

- **crypto/** : Contient les modules liés au chiffrement et à la gestion des clés.
  - `secure_channel.rs` : Implémente le chiffrement et le déchiffrement des messages.
  - `key.rs` : Gère la clé partagée utilisée pour le chiffrement.
- **server/** : Regroupe la logique du serveur.
  - `client_info.rs` : Structure et logique anti-spam pour les clients.
  - `commands.rs` : Gestion des commandes utilisateur.
  - `handlers.rs` : Fonctions principales pour gérer les connexions et la diffusion des messages.
  - `username.rs` : Gestion de la requête et de l'enregistrement des noms d'utilisateur.
- **util/** : Contient des fonctions utilitaires.
  - `send_encrypted.rs` : Fonction pour envoyer des messages chiffrés.
- **logger.rs** : Fonctions de journalisation des événements.
- **messages.rs** : Définitions des messages utilisés dans l'application.
- **main.rs** : Point d'entrée du serveur.
- **bin/client.rs** : Point d'entrée du client.

## Installation

### Prérequis

- **Rust** : Assurez-vous d'avoir Rust installé. Vous pouvez l'installer via [rustup](https://rustup.rs/).

### Étapes d'Installation

1. **Cloner le Répertoire**

   ```bash
   git clone git@github.com:MatthieuBarraque/Rust-Cipher-SecureComms.git
   cd Rust-Cipher-SecureComms
   ```

2. **Compiler le Projet**

   Utilisez Cargo pour compiler le serveur et le client.

   ```bash
   cargo build --release
   ```

   Les binaires seront situés dans `target/release/`.

## Utilisation

### Lancer le Serveur

Dans un terminal, exécutez le serveur :

```bash
cargo run --bin secure_chat
```

Vous verrez des messages indiquant que le serveur a démarré et est en écoute.

### Lancer le Client

Dans un autre terminal, exécutez le client :

```bash
cargo run --bin client
```

Suivez les instructions à l'écran pour entrer votre nom d'utilisateur et commencer à envoyer des messages.

### Commandes Disponibles

- `/help` : Affiche les commandes disponibles.
- `/list` : Liste tous les utilisateurs actuellement connectés.
- `/quit` : Se déconnecter du serveur.

## Configuration

Le projet peut être configuré via un fichier `config.toml` (optionnel). Vous pouvez définir des paramètres tels que l'adresse du serveur, les ports, etc.

Exemple de `config.toml` :

```toml
[server]
address = "127.0.0.1"
port = 7878

[security]
max_messages_per_sec = 5
```

*(Ajoutez ce fichier dans la racine du projet et modifiez `main.rs` et `client.rs` pour lire ces configurations.)*

## Sécurité

**Secure Messaging** intègre plusieurs mécanismes de sécurité pour garantir la confidentialité et l'intégrité des communications :

- **Chiffrement AES-256-GCM** : Utilisé pour chiffrer et déchiffrer les messages entre les clients et le serveur.
- **Gestion des Clés** : Une clé partagée est utilisée pour le chiffrement symétrique. La clé est stockée de manière sécurisée et est effacée lors de la fermeture du serveur.
- **Anti-Spam** : Limitation du nombre de messages qu'un utilisateur peut envoyer par seconde pour prévenir les abus.
- **Validation des Messages** : Tous les messages sont décodés et déchiffrés avant d'être traités. En cas d'échec, des messages d'erreur sécurisés sont envoyés aux clients.
- **Journalisation Sécurisée** : Les événements importants sont enregistrés avec des niveaux de gravité appropriés (INFO, WARN, ERREUR).

### Améliorations Futures

Pour renforcer davantage la sécurité, envisagez d'ajouter :

- **Échange de Clés Sécurisé** : Implémenter un protocole d'échange de clés (comme Diffie-Hellman) pour générer des clés de session uniques par client.
- **Authentification des Clients** : Ajouter une étape d'authentification avec des identifiants uniques pour chaque utilisateur.
- **Chiffrement de Bout en Bout** : Assurer que seuls l'expéditeur et le destinataire peuvent lire les messages.
- **Utilisation de TLS** : Chiffrer les communications réseau avec TLS pour protéger contre les interceptions.
