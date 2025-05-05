mod compilers;
mod utils;

use clap::{Parser, Subcommand};
use colored::*;
use compilers::{golang, java, python};
use std::{thread, time};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Install development tools. Use `dev install --help` for available options")]
    Install {
        #[clap(long, help = "Install pip")]
        pip: bool,
        #[clap(long, help = "Install virtualenv")]
        virtualenv: bool,
        #[clap(long, help = "Install Go")]
        go: bool,
        #[clap(long, help = "Install JDK")]
        jdk: bool,
        #[clap(long, help = "Install OpenJFX")]
        openjfx: bool,
        #[clap(long, help = "Install dotnet-sdk")]
        dotnet: bool,
        #[clap(long, help = "Install NodeJS and npm")]
        nodejs: bool,
        #[clap(long, help = "Install yarn")]
        yarn: bool,
        #[clap(long, help = "Install Rust")]
        rust: bool,
        #[clap(long, help = "Install Docker")]
        docker: bool,
        #[clap(long, help = "Install all tools")]
        all: bool,
    },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Install {
            pip,
            virtualenv,
            go,
            jdk,
            openjfx,
            dotnet,
            nodejs,
            yarn,
            rust,
            docker,
            all,
        } => {
            if all || pip {
                python::install_pip();
            }
            if all || virtualenv {
                python::install_virtualenv();
            }
            if all || go {
                golang::install_go();
            }
            if all || jdk {
                java::install_jdk();
            }
            if all || openjfx {
                java::install_openjfx();
            }
            if all || dotnet {
                install_dotnet();
            }
            if all || nodejs {
                install_nodejs();
            }
            if all || yarn {
                install_yarn();
            }
            if all || rust {
                install_rust();
            }
            if all || docker {
                install_docker();
            }
        }
    }
}

fn install_dotnet() {
    println!("{}", "Installing dotnet-sdk...".blue());
    utils::run_command(&["sudo", "pamac", "install", "dotnet-sdk"]);
    utils::run_command(&["dotnet", "--version"]);
    println!("{}", "dotnet-sdk installation complete".blue());
    thread::sleep(time::Duration::from_secs(2));
}

fn install_nodejs() {
    println!(
        "{}",
        "Installing the Latest version of NodeJS and npm...".blue()
    );
    utils::run_command(&["sudo", "pacman", "-S", "nodejs", "npm"]);
    utils::run_command(&["node", "-v"]);
    utils::run_command(&["npm", "-v"]);
    println!("{}", "NodeJS (Latest) installation complete".blue());
    thread::sleep(time::Duration::from_secs(2));
}

fn install_yarn() {
    println!("{}", "Installing yarn...".blue());
    utils::run_command(&["npm", "config", "set", "prefix", "~/.npm-global"]);
    utils::run_command(&["npm", "install", "--global", "yarn"]);
    utils::run_command(&["yarn", "--version"]);
    println!("{}", "yarn installation complete".blue());
    thread::sleep(time::Duration::from_secs(2));
}

fn install_rust() {
    println!("{}", "Installing Rust toolchain".blue());
    utils::run_command(&[
        "curl",
        "--proto",
        "=https",
        "--tlsv1.2",
        "-sSf",
        "https://sh.rustup.rs",
        "|",
        "sh",
        "-s",
        "--",
        "-y",
    ]);
    utils::run_command(&["rustc", "--version"]);
    utils::run_command(&["cargo", "--version"]);
    println!("{}", "Rust toolchain installation complete".blue());
    thread::sleep(time::Duration::from_secs(2));
}

fn install_docker() {
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
