mod compilers;

use clap::{Parser, Subcommand};
use colored::*;
use compilers::python;
use regex::Regex;
use reqwest;
use scraper::{Html, Selector};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::{thread, time};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
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

        #[clap(long, help = "Get OpenJFX Version")]
        jfxver: bool,
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
            jfxver,
        } => {
            if all || pip {
                python::install_pip();
            }
            if all || virtualenv {
                python::install_virtualenv();
            }
            if all || go {
                install_go();
            }
            if all || jdk {
                install_jdk();
            }
            if all || openjfx {
                install_openjfx().await;
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
            if all || jfxver {
                get_latest_openjfx_version()
                    .await
                    .expect("LOG: Failed to get OpenJFX version");
            }
        }
    }
}

fn install_go() {
    println!("Installing Latest Go...");

    // Fetch the latest Go version
    let version_output = Command::new("curl")
        .args(&["-s", "https://go.dev/VERSION?m=text"])
        .output()
        .expect("Failed to get Go version");

    let version_string = String::from_utf8_lossy(&version_output.stdout);

    // Extract only the first line, which contains the version number
    let version = version_string
        .lines()
        .next()
        .unwrap_or("")
        .trim()
        .to_string();

    if !version.starts_with("go") {
        panic!("Invalid Go version string received: {}", version);
    }

    let filename = format!("{}.linux-amd64.tar.gz", version);
    let url = format!("https://dl.google.com/go/{}", filename);

    println!("Downloading Go version: {} from {}", version, url);

    // Download the tarball
    run_command(&["curl", "-L", "-o", &filename, &url]);

    // Remove existing Go installation
    if Path::new("/usr/local/go").exists() {
        run_command(&["sudo", "rm", "-rf", "/usr/local/go"]);
    }

    // Extract Go tarball
    run_command(&["sudo", "tar", "-C", "/usr/local", "-xzf", &filename]);

    // Clean up tarball
    if Path::new(&filename).exists() {
        fs::remove_file(&filename).expect("Failed to remove tarball");
    }

    // Set up Go environment variables
    let profile_path = "/etc/profile.d/go.sh";
    let go_path_export = "export PATH=$PATH:/usr/local/go/bin";

    if !Path::new(profile_path).exists() {
        run_command(&[
            "sudo",
            "bash",
            "-c",
            &format!("echo '{}' > {}", go_path_export, profile_path),
        ]);
        run_command(&["sudo", "chmod", "+x", profile_path]);
    }

    // Source the profile
    run_command(&["bash", "-c", "source /etc/profile.d/go.sh"]);

    // Verify the installation
    run_command(&["go", "version"]);

    println!("Go installation complete!");
    thread::sleep(time::Duration::from_secs(2));
}

fn install_jdk() {
    println!("{}", "Installing Latest Java (JDK)...".blue());

    // Fetch the Oracle Java Downloads page
    let output = Command::new("curl")
        .args(&[
            "-s",
            "https://www.oracle.com/bd/java/technologies/downloads/",
        ])
        .output()
        .expect("Failed to fetch JDK download page");

    let output_string = String::from_utf8_lossy(&output.stdout);

    // Regex to extract the latest Java version from the specific line
    let version_regex =
        Regex::new(r"Java (\d+), Java \d+, and earlier versions available now").unwrap();

    if let Some(captures) = version_regex.captures(&output_string) {
        if let Some(version_match) = captures.get(1) {
            let version = version_match.as_str();
            println!("Installing Java (JDK) version: {}", version);

            // Construct the download URL for the latest JDK
            let filename = format!("jdk-{}_linux-x64_bin.tar.gz", version);
            let url = format!(
                "https://download.oracle.com/java/{}/latest/{}",
                version, filename
            );

            // Download the JDK tarball
            run_command(&["curl", "-O", &url]);

            // Extract the tarball to /usr/local
            run_command(&["sudo", "tar", "-C", "/usr/local", "-xzf", &filename]);

            // Verify the installation
            run_command(&["/usr/local/jdk/bin/java", "-version"]);

            println!("{}", "JDK installation complete".blue());
            thread::sleep(time::Duration::from_secs(2));
        } else {
            eprintln!("{}", "Error: Could not extract Java version.".red());
        }
    } else {
        eprintln!(
            "{}",
            "Error: Could not find Java versions line on the page.".red()
        );
    }
}

async fn get_latest_openjfx_version() -> Result<Option<String>, reqwest::Error> {
    let url = "https://gluonhq.com/products/javafx/";
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        return Ok(None);
    }

    let body = response.text().await?;
    let document = Html::parse_document(&body);
    let table_selector = Selector::parse("table#roadmap").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    if let Some(table) = document.select(&table_selector).next() {
        for row in table.select(&row_selector) {
            let cells: Vec<_> = row
                .select(&cell_selector)
                .map(|c| c.text().collect::<String>())
                .collect();

            if cells.len() > 2 {
                let version_text = cells[2].trim().to_string();

                if !version_text.contains("early access") {
                    let version_number = version_text
                        .split_whitespace()
                        .next()
                        .unwrap_or("")
                        .to_string();

                    return Ok(Some(version_number));
                }
            }
        }
    } else {
        println!("Could not find roadmap table.");
    }

    Ok(None)
}

async fn install_openjfx() {
    println!("{}", "Installing Latest OpenJFX...".blue());

    match get_latest_openjfx_version().await {
        Ok(Some(version)) => {
            println!("Latest OpenJFX version: {}", version);

            let filename = format!("openjfx-{}_linux-x64_bin-sdk.zip", version);
            let url = format!(
                "https://download2.gluonhq.com/openjfx/{}/{}",
                version, filename
            );

            run_command(&["curl", "-O", &url]);
            run_command(&["sudo", "unzip", &filename, "-d", "/usr/local"]);

            println!("{}", "OpenJFX installation complete!".blue());
            // thread::sleep(time::Duration::from_secs(2));
        }
        Ok(None) => {
            eprintln!("{}", "Error: Could not find latest OpenJFX version.".red());
        }
        Err(e) => {
            eprintln!("Error fetching OpenJFX version: {}", e);
        }
    }
}

fn install_dotnet() {
    println!("{}", "Installing dotnet-sdk...".blue());
    run_command(&["sudo", "pamac", "install", "dotnet-sdk"]);
    run_command(&["dotnet", "--version"]);
    println!("{}", "dotnet-sdk installation complete".blue());
    thread::sleep(time::Duration::from_secs(2));
}

fn install_nodejs() {
    println!(
        "{}",
        "Installing the Latest version of NodeJS and npm...".blue()
    );
    run_command(&["sudo", "pacman", "-S", "nodejs", "npm"]);
    run_command(&["node", "-v"]);
    run_command(&["npm", "-v"]);
    println!("{}", "NodeJS (Latest) installation complete".blue());
    thread::sleep(time::Duration::from_secs(2));
}

fn install_yarn() {
    println!("{}", "Installing yarn...".blue());
    run_command(&["npm", "config", "set", "prefix", "~/.npm-global"]);
    run_command(&["npm", "install", "--global", "yarn"]);
    run_command(&["yarn", "--version"]);
    println!("{}", "yarn installation complete".blue());
    thread::sleep(time::Duration::from_secs(2));
}

fn install_rust() {
    println!("{}", "Installing Rust toolchain".blue());
    run_command(&[
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
    run_command(&["rustc", "--version"]);
    run_command(&["cargo", "--version"]);
    println!("{}", "Rust toolchain installation complete".blue());
    thread::sleep(time::Duration::from_secs(2));
}

fn install_docker() {
    println!("{}", "Installing Docker...".blue());
    run_command(&[
        "curl",
        "-O",
        "https://desktop.docker.com/linux/main/amd64/docker-desktop-x86_64.pkg.tar.zst",
    ]);
    run_command(&[
        "sudo",
        "pacman",
        "-U",
        "./docker-desktop-x86_64.pkg.tar.zst",
    ]);
    run_command(&["docker", "version"]);
    run_command(&["docker", "compose", "version"]);
    println!("{}", "Docker installation complete".blue());
    thread::sleep(time::Duration::from_secs(2));
}

fn run_command(command: &[&str]) {
    let mut cmd = Command::new(command[0]);
    cmd.args(&command[1..]);
    match cmd.status() {
        Ok(status) if status.success() => {
            if command[0] != "curl" && command[0] != "sudo" {
                let output = cmd.output().expect("failed to execute command");
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
        }
        Ok(status) => {
            eprintln!("Command failed: {}", status);
        }
        Err(e) => {
            eprintln!("Error running command: {}", e);
        }
    }
}
