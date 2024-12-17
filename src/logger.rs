
pub fn info(msg: &str) {
    println!("\x1b[1;32m[INFO]\x1b[0m {}", msg);
}

pub fn warn(msg: &str) {
    eprintln!("\x1b[1;33m[WARN]\x1b[0m {}", msg);
}

pub fn error(msg: &str) {
    eprintln!("\x1b[1;31m[ERREUR]\x1b[0m {}", msg);
}
