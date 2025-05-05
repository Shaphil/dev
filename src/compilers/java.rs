use crate::utils;
use colored::Colorize;
use std::io::Write;
use std::{io, thread, time};

// TODO: Get version number manually and load from a `.env` or a `config.json` file
pub fn install_jdk() {
    // TODO: move available versions to a `config.json` file
    let available_versions = vec!["23.0.2", "21"];
    println!("{}", "Available JDK versions:".bold());
    for (index, version) in available_versions.iter().enumerate() {
        println!("{}. {}", index + 1, version);
    }

    loop {
        print!("{}", "Please choose a version to install: ".bold());
        io::stdout().flush().unwrap(); // ensure the prompt is displayed

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Invalid value for JDK version number");

        let input = input.trim();
        match input.parse::<usize>() {
            Ok(version) if (1..=available_versions.len()).contains(&version) => {
                let selected_version = available_versions[version - 1];
                println!("{} {}", "Installing JDK version:".blue(), selected_version);

                let filename = format!("jdk-{}_linux-x64_bin.tar.gz", selected_version);
                let url = format!(
                    "https://download.oracle.com/java/{}/latest/{}",
                    selected_version.split(".").next().unwrap(),
                    filename
                );

                println!("Downloading: {}", url);

                utils::run_command(&["curl", "-O", &url]);
                utils::run_command(&["sudo", "tar", "-C", "/usr/local", "-xzf", &filename]);

                utils::run_command(&["/usr/local/jdk/bin/java", "-version"]);

                println!("{}", "JDK installation complete".blue());
                thread::sleep(time::Duration::from_secs(2));
            }
            Ok(_) | Err(_) => {
                println!(
                    "{}",
                    "Invalid choice. Please enter a number from the list.".red()
                );
            }
        }
    }
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

                utils::run_command(&["curl", "-O", &url]);
                utils::run_command(&["sudo", "unzip", &filename, "-d", "/usr/local"]);
                println!("{}", "OpenJFX installation complete!".blue());
                thread::sleep(time::Duration::from_secs(2));
            }
            _ => {
                println!("{}", "Invalid choice".red());
            }
        }
    }
}
