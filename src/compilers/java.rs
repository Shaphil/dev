use crate::utils;
use colored::Colorize;
use scraper::{Html, Selector};
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

pub async fn get_latest_openjfx_version() -> Result<Option<String>, reqwest::Error> {
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

// TODO: Get version number manually and load from a `.env` or a `config.json` file
pub async fn install_openjfx() {
    println!("{}", "Installing Latest OpenJFX...".blue());

    match get_latest_openjfx_version().await {
        Ok(Some(version)) => {
            println!("Latest OpenJFX version: {}", version);

            let filename = format!("openjfx-{}_linux-x64_bin-sdk.zip", version);
            let url = format!(
                "https://download2.gluonhq.com/openjfx/{}/{}",
                version, filename
            );

            utils::run_command(&["curl", "-O", &url]);
            utils::run_command(&["sudo", "unzip", &filename, "-d", "/usr/local"]);

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
