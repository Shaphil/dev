use colored::Colorize;
use std::process::Command;
use std::{thread, time};

pub enum CommandExecutionStatus {
    SUCCEEDED,
    FAILED,
}

pub fn run_command(command: &[&str]) -> CommandExecutionStatus {
    let mut cmd = Command::new(command[0]);
    cmd.args(&command[1..]);
    match cmd.status() {
        Ok(status) if status.success() => {
            if command[0] != "curl" && command[0] != "sudo" {
                let output = cmd.output().expect("failed to execute command");
                println!("{}", String::from_utf8_lossy(&output.stdout).green());
            }
            CommandExecutionStatus::SUCCEEDED
        }
        Ok(status) => {
            let output = format!("Command failed: {}", status);
            eprintln!("{}", output.red());
            CommandExecutionStatus::FAILED
        }
        Err(e) => {
            let output = format!("Error running command: {}", e);
            eprintln!("{}", output.red());
            CommandExecutionStatus::FAILED
        }
    }
}

pub fn sleep(secs: u64) {
    thread::sleep(time::Duration::from_secs(secs));
}

pub struct DevTool {
    tool: String,
    executor: String,
    description: String,
    is_sudo: bool,
}

impl DevTool {
    pub fn new(tool: String, executor: String, description: String, is_sudo: bool) -> Self {
        Self {
            tool,
            executor,
            description,
            is_sudo,
        }
    }

    // pub fn download(&mut self, url: &str) {
    //     run_command(&["curl", "-O", url]);
    // }

    pub fn custom_install(&mut self, cmd: &str) {
        run_command(&["bash", "-c", cmd]);
    }

    pub fn install(&mut self) {
        println!(
            "{} {} {}",
            "Installing".blue(),
            self.tool.blue().bold(),
            self.description.blue()
        );
        let status;
        if self.is_sudo {
            status = run_command(&[
                "sudo",
                &self.executor.to_string(),
                "-S",
                &self.tool.to_string(),
            ]);
        } else {
            status = run_command(&[&self.executor.to_string(), "-S", &self.tool.to_string()]);
        }
        print_status(&self.tool.to_string(), status);
        sleep(2);
    }
}

fn print_status(tool: &str, status: CommandExecutionStatus) {
    match status {
        CommandExecutionStatus::SUCCEEDED => {
            println!(
                "{} {}",
                tool.green().bold(),
                "installation complete".green()
            )
        }
        CommandExecutionStatus::FAILED => {
            println!("{} {}", tool.green().bold(), "installation failed".red())
        }
    }
}
