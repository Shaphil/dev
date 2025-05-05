use crate::utils;
use colored::Colorize;
use std::{thread, time};

pub fn install_docker() {
    println!("{}", "Installing Docker...".blue());
    utils::run_command(&[
        "curl",
        "-O",
        "https://desktop.docker.com/linux/main/amd64/docker-desktop-x86_64.pkg.tar.zst",
    ]);
    utils::run_command(&[
        "sudo",
        "pacman",
        "-U",
        "./docker-desktop-x86_64.pkg.tar.zst",
    ]);
    utils::run_command(&["docker", "version"]);
    utils::run_command(&["docker", "compose", "version"]);
    println!("{}", "Docker installation complete".blue());
    thread::sleep(time::Duration::from_secs(2));
}
