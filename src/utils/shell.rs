use crate::utils::logger;
use colored::Colorize;
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

pub fn append_to_shell_config(
    config: &str,
    rc_filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let home = dirs::home_dir().ok_or("Could not find home directory")?;
    let rc_path: PathBuf = home.join(rc_filename);

    // 1. Read existing lines into a HashSet
    let mut existing_lines = HashSet::new();
    if rc_path.exists() {
        let file = fs::File::open(&rc_path)?;
        let reader = BufReader::new(file);
        for line_result in reader.lines() {
            existing_lines.insert(line_result?);
        }
    }

    // 2. Check if the exact line is already in the set (O(1) lookup)
    if existing_lines.contains(config) {
        println!("Configuration already exists in {}, skipping.", rc_filename);
        return Ok(());
    }

    // 3. If it doesn't exist, append it to the file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&rc_path)?;

    writeln!(file, "{}", config)?;
    logger::info(&format!("Successfully updated {}", rc_filename));

    Ok(())
}

pub fn config_already_exists(
    anchor: &str,
    rc_filename: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let home = dirs::home_dir().ok_or("Could not find home directory")?;
    let rc_path = home.join(rc_filename);

    if !rc_path.exists() {
        return Ok(false);
    }

    let file = fs::File::open(&rc_path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        // If any line contains our unique anchor, consider it already installed
        if line.contains(anchor) {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn update_shell_configs(
    config: &str,
    anchor: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let home = dirs::home_dir().ok_or("Could not find home directory")?;
    let rc_files = [".bashrc", ".zshrc"];

    for rc_file in rc_files {
        let rc_path = home.join(rc_file);

        // 1. Guard clause: Skip if the file doesn't exist
        if !rc_path.exists() {
            continue;
        }

        // 2. Guard clause: Skip if the config anchor is already there
        if config_already_exists(anchor, rc_file)? {
            return Ok("config already exists in `.zshrc|.bashrc`"
                .blue()
                .to_string());
        }

        // 3. If missing, append it
        append_to_shell_config(config, rc_file)?;
    }

    Ok("added to `.zshrc|.bashrc`".green().to_string())
}
