mod compilers;
mod devops;
mod misc;
mod utils;

use clap::{Parser, Subcommand, ValueEnum};
use compilers::{dotnet, golang, java, js, python, rust};
use devops::docker;
use misc::{nerd_fonts, oh_my_posh};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Install development tools. Use `dev install --help` for available options
    Install {
        /// The tool to install
        #[arg(value_enum, num_args = 1.., required = true)]
        tools: Vec<Tool>,
    },
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum Tool {
    /// Install pip
    Pip,
    /// Install virtualenv
    Virtualenv,
    /// Install Go
    Go,
    /// Install JDK
    Jdk,
    /// Install OpenJFX
    Openjfx,
    /// Install dotnet-sdk
    Dotnet,
    /// Install NodeJS and npm
    Nodejs,
    /// Install yarn
    Yarn,
    /// Install Rust
    Rust,
    /// Install Docker
    Docker,
    /// Nerd Fonts
    Nerdfonts,
    /// Oh-my-posh
    OhMyPosh,
    /// Install all tools
    All,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let args = Args::parse();
    match args.command {
        Commands::Install { tools } => {
            let tools_to_install = if tools.contains(&Tool::All) {
                vec![
                    Tool::Pip,
                    Tool::Virtualenv,
                    Tool::Go,
                    Tool::Jdk,
                    Tool::Openjfx,
                    Tool::Dotnet,
                    Tool::Nodejs,
                    Tool::Yarn,
                    Tool::Rust,
                    Tool::Docker,
                    Tool::Nerdfonts,
                    Tool::OhMyPosh,
                ]
            } else {
                tools
            };

            // Loop through and install each one
            for t in tools_to_install {
                match t {
                    Tool::Pip => python::install_pip(),
                    Tool::Virtualenv => python::install_virtualenv(),
                    Tool::Go => golang::install_go(),
                    Tool::Jdk => java::install_jdk(),
                    Tool::Openjfx => java::install_openjfx(),
                    Tool::Dotnet => dotnet::install_dotnet(),
                    Tool::Nodejs => js::install_nodejs(),
                    Tool::Yarn => js::install_yarn(),
                    Tool::Rust => rust::install_rust(),
                    Tool::Docker => docker::install_docker(),
                    Tool::Nerdfonts => nerd_fonts::install_nerd_fonts(),
                    Tool::OhMyPosh => oh_my_posh::install_oh_my_posh(),
                    Tool::All => unreachable!(),
                }
            }
        }
    }
}
