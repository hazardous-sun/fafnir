use colored::Colorize;

pub fn info(message: &str) {
    println!("{}: {}", "INFO".cyan(), message);
}

pub fn warning(message: &str) {
    println!("{}: {}", "WARNING".yellow(), message);
}

pub fn error(message: &str) {
    println!("{}: {}", "ERROR".red(), message);
}

pub fn debug(message: &str) {
    println!("{}: {}", "DEBUG".truecolor(135, 255, 135), message);
}
