use colored::Colorize;

pub fn info(message: &str) {
    println!("INFO: {}", message.cyan());
}

pub fn warn(message: &str) {
    println!("WARNING: {}", message.yellow());
}

pub fn error(message: &str) {
    println!("ERROR: {}", message.red());
}
