use colored::Colorize;
use std::sync::atomic::{AtomicBool, Ordering};

static VERBOSE_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn init_verbose(enabled: bool) {
    VERBOSE_ENABLED.store(enabled, Ordering::Relaxed);
}

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
    if VERBOSE_ENABLED.load(Ordering::Relaxed) {
        println!("{}: {}", "DEBUG".truecolor(135, 255, 135), message);
    }
}
