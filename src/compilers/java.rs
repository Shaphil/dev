use crate::utils::cmd;
use crate::utils::cmd::DevTool;
use colored::Colorize;
use std::io::Write;
use std::process::Command;
use std::{io, thread, time};

pub fn install_jdk() {
    // Dynamically fetch available LTS versions from Adoptium API
    let output = Command::new("sh")
        .arg("-c")
        .arg("curl -s https://api.adoptium.net/v3/info/available_releases | jq -r '.available_lts_releases[]'")
        .output();

    let available_versions: Vec<String> = match output {
        Ok(res) if res.status.success() => {
            let stdout = String::from_utf8_lossy(&res.stdout);
            stdout
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        }
        _ => {
            // Fallback jq fails
            vec!["21".to_string(), "25".to_string()]
        }
    };

    println!("{}", "Available JDK versions:".bold());
    for (index, version) in available_versions.iter().enumerate() {
        println!("{}. JDK {}", index + 1, version);
    }

    let selected_version;
    let tool;
    let mut dev_tool;
    loop {
        print!("{}", "Please choose a version to install: ".bold());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Invalid value for JDK version number");

        let input = input.trim();
        match input.parse::<usize>() {
            Ok(version) if (1..=available_versions.len()).contains(&version) => {
                selected_version = &available_versions[version - 1];
                println!("{} {}", "Installing JDK version:".blue(), selected_version);

                // Use the Adoptium API URL pattern instead of Oracle's tarball link
                let url = format!(
                    "https://api.adoptium.net/v3/binary/latest/{}/ga/linux/x64/jdk/hotspot/normal/eclipse",
                    selected_version
                );
                println!("Downloading: {}", url);

                tool = format!("OpenJDK-{}", selected_version);
                let cmd = format!(
                    "mkdir -p ~/.local/opt && \
                     curl -sL {} -o openjdk.tar.gz && \
                     tar -xzf openjdk.tar.gz -C ~/.local/opt/ && \
                     rm -rf ~/.local/opt/java && \
                     mv ~/.local/opt/jdk-* ~/.local/opt/java && \
                     rm openjdk.tar.gz",
                    url
                );

                dev_tool = DevTool::new(
                    tool.clone(),
                    "sh -c".to_string(),
                    "Eclipse Temurin OpenJDK".to_string(),
                    false,
                );
                dev_tool.run(&cmd);

                println!("{}", "JDK installation complete".blue());
                thread::sleep(time::Duration::from_secs(2));
                break;
            }
            _ => {
                println!(
                    "{}",
                    "Invalid choice. Please enter a number from the list.".red()
                );
            }
        }
    }
    // Configure JAVA_HOME and PATH in shell configuration files
    let anchor = "Java";
    let config_lines = [
        "",
        "# Java Environment Variables",
        "export JAVA_HOME=\"$HOME/.local/opt/java\"",
        "export PATH=\"$JAVA_HOME/bin:$PATH\"",
    ];
    let config = config_lines.join("\n");
    dev_tool.update_shell(config, anchor);
    dev_tool.check("javac -version");
}

// TODO: Get version number manually and load from a `.env` or a `config.json` file
pub fn install_openjfx() {
    // TODO: move available versions to a `config.json` file
    let available_versions = vec!["24.0.1", "21.0.7", "17.0.15"];
    println!("{}", "Available JavaFX versions:".bold());
    for (index, version) in available_versions.iter().enumerate() {
        println!("{}. {}", index + 1, version);
    }

    loop {
        println!("{}", "Please choose a JavaFX version to install: ".blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid choice!");

        let input = input.trim();
        match input.parse::<usize>() {
            Ok(version) if (1..=available_versions.len()).contains(&version) => {
                let selected_version = available_versions[version - 1];
                println!(
                    "{} {}{}",
                    "Installing OpenJFX".blue(),
                    "v",
                    selected_version
                );
                let filename = format!("openjfx-{}_linux-x64_bin-sdk.zip", selected_version);
                let url = format!(
                    "https://download2.gluonhq.com/openjfx/{}/{}",
                    selected_version, filename
                );

                println!("filename: {}", filename);
                println!("URL: {}", url);

                cmd::run_command(&["curl", "-O", &url]);
                cmd::run_command(&["sudo", "unzip", &filename, "-d", "/usr/local"]);
                println!("{}", "OpenJFX installation complete!".blue());
                thread::sleep(time::Duration::from_secs(2));
            }
            _ => {
                println!("{}", "Invalid choice".red());
            }
        }
    }
}
