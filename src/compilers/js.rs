use crate::utils::cmd;
use colored::Colorize;
use std::{thread, time};

// TODO: install from tarball to avoid `pacman`, `https://nodejs.org/en/download`
pub fn install_nodejs() {
    println!(
        "{}",
        "Installing the Latest version of NodeJS and npm...".blue()
    );
    cmd::run_command(&["sudo", "pacman", "-S", "nodejs", "npm"]);
    cmd::run_command(&["node", "-v"]);
    cmd::run_command(&["npm", "-v"]);
    println!("{}", "NodeJS (Latest) installation complete".blue());
    thread::sleep(time::Duration::from_secs(2));
}

pub fn install_yarn() {
    println!("{}", "Installing yarn...".blue());
    cmd::run_command(&["npm", "config", "set", "prefix", "~/.npm-global"]);
    cmd::run_command(&["npm", "install", "--global", "yarn"]);
    cmd::run_command(&["yarn", "--version"]);
    println!("{}", "yarn installation complete".blue());
    thread::sleep(time::Duration::from_secs(2));
}
