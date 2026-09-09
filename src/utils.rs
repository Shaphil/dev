use colored::Colorize;
use std::process::Command;
use std::{thread, time};

pub enum CommandStatus {
    SUCCEEDED,
    FAILED,
}

pub fn run_command(command: &[&str]) -> CommandStatus {
    let mut cmd = Command::new(command[0]);
    cmd.args(&command[1..]);
    match cmd.status() {
        Ok(status) if status.success() => {
            if command[0] != "curl" && command[0] != "sudo" {
                let output = cmd.output().expect("failed to execute command");
                println!("{}", String::from_utf8_lossy(&output.stdout).green());
            }
            CommandStatus::SUCCEEDED
        }
        Ok(status) => {
            let output = format!("Command failed: {}", status);
            eprintln!("{}", output.red());
            CommandStatus::FAILED
        }
        Err(e) => {
            let output = format!("Error running command: {}", e);
            eprintln!("{}", output.red());
            CommandStatus::FAILED
        }
    }
}

pub fn sleep(secs: u64) {
    thread::sleep(time::Duration::from_secs(secs));
}
