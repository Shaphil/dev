use colored::Colorize;
use std::{thread, time};

use crate::utils;

pub fn install_pip() {
    println!("{}", "Installing Python pip...".blue());
    utils::run_command(&["sudo", "pacman", "-S", "python-pip"]);
    utils::run_command(&["pip", "--version"]);
    println!("{}", "pip installation complete".blue());
    thread::sleep(time::Duration::from_secs(2));
}

pub fn install_virtualenv() {
    println!("{}", "Installing Virtualenv...".blue());
    utils::run_command(&["pip", "install", "virtualenv", "--break-system-packages"]);
    utils::run_command(&["virtualenv", "--version"]);
    println!("{}", "virtualenv installation complete".blue());
    thread::sleep(time::Duration::from_secs(2));
}
