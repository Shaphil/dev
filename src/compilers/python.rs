use colored::Colorize;
use std::{thread, time};

use crate::utils::cmd;
use crate::utils::cmd::DevTool;

pub fn install_pip() {
    println!("{}", "Installing Python pip...".blue());
    cmd::run_command(&["sudo", "pacman", "-S", "python-pip"]);
    cmd::run_command(&["pip", "--version"]);
    println!("{}", "pip installation complete".blue());
    thread::sleep(time::Duration::from_secs(2));
}

pub fn install_virtualenv() {
    println!("{}", "Installing Virtualenv...".blue());
    cmd::run_command(&["pip", "install", "virtualenv", "--break-system-packages"]);
    cmd::run_command(&["virtualenv", "--version"]);
    println!("{}", "virtualenv installation complete".blue());
    thread::sleep(time::Duration::from_secs(2));
}

pub fn install_uv() {
    let tool = "uv".to_string();
    let executor = "".to_string();
    let description =
        "An extremely fast Python package and project manager, written in Rust.".to_string();
    let is_sudo = false;
    let mut cmd = "curl -LsSf https://astral.sh/uv/install.sh | sh";

    let mut dev_tool = DevTool::new(tool, executor, description, is_sudo);
    dev_tool.run(cmd);

    cmd = "uv --version";
    dev_tool.check(cmd);
}
