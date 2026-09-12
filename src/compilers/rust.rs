use crate::utils::cmd;
use colored::Colorize;
use std::{thread, time};

pub fn install_rust() {
    println!("{}", "Installing Rust toolchain".blue());
    let command = "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y";
    cmd::run_command(&["sh", "-c", command]);
    cmd::run_command(&["rustc", "--version"]);
    cmd::run_command(&["cargo", "--version"]);
    println!("{}", "Rust toolchain installation complete".blue());
    thread::sleep(time::Duration::from_secs(2));
}
