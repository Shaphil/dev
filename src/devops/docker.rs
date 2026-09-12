use crate::utils::cmd;
use colored::Colorize;
use std::{thread, time};

pub fn install_docker() {
    println!("{}", "Installing Docker...".blue());
    cmd::run_command(&[
        "curl",
        "-O",
        "https://desktop.docker.com/linux/main/amd64/docker-desktop-x86_64.pkg.tar.zst",
    ]);
    cmd::run_command(&[
        "sudo",
        "pacman",
        "-U",
        "./docker-desktop-x86_64.pkg.tar.zst",
    ]);
    cmd::run_command(&["docker", "version"]);
    cmd::run_command(&["docker", "compose", "version"]);
    println!("{}", "Docker installation complete".blue());
    thread::sleep(time::Duration::from_secs(2));
}
