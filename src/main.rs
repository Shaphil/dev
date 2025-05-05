mod compilers;
mod utils;

use clap::{Parser, Subcommand};
use colored::*;
use compilers::{dotnet, golang, java, js, python, rust};
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
                dotnet::install_dotnet();
            }
            if all || nodejs {
                js::install_nodejs();
            }
            if all || yarn {
                js::install_yarn();
            }
            if all || rust {
                rust::install_rust();
            }
            if all || docker {
                install_docker();
            }
        }
    }
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
