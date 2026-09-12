use colored::Colorize;

pub fn info(message: &str) {
    println!("{} {}", "[INFO]".blue().bold(), message);
}

pub fn success(message: &str) {
    println!("{} {}", "[SUCCESS]".green().bold(), message);
}

pub fn warning(message: &str) {
    eprintln!("{} {}", "[WARNING]".yellow().bold(), message);
}

pub fn error(message: &str) {
    eprintln!("{} {}", "[ERROR]".red().bold(), message);
}
