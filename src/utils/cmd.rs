use crate::utils::logger;
use crate::utils::shell::update_shell_configs;
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

    match cmd.output() {
        Ok(output) if output.status.success() => {
            let stdout_str = String::from_utf8_lossy(&output.stdout);

            if !stdout_str.is_empty() {
                if command[0] != "curl" && command[0] != "sudo" {
                    logger::success(&*stdout_str)
                } else {
                    logger::info(&*stdout_str);
                }
            }

            CommandExecutionStatus::SUCCEEDED
        }
        Ok(output) => {
            let err_str = String::from_utf8_lossy(&output.stderr);
            let msg = if err_str.is_empty() {
                format!("Command failed with exit code: {}", output.status)
            } else {
                format!("Command failed: {}", err_str)
            };
            logger::error(&*msg);
            CommandExecutionStatus::FAILED
        }
        Err(e) => {
            let output = format!("Error running command: {}", e);
            logger::error(&*output);
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

    fn print_installation_info(&self) {
        let message = format!(
            "{} {} {}",
            "Installing".blue(),
            self.tool.blue().bold(),
            self.description.blue()
        );
        logger::info(&message);
    }

    pub fn run(&mut self, cmd: &str) {
        self.print_installation_info();

        let status;
        if self.is_sudo {
            status = run_command(&["sudo", "bash", "-c", cmd]);
        } else {
            status = run_command(&["bash", "-c", cmd]);
        }

        self.print_status(status);
        sleep(2);
    }

    pub fn check(&mut self, cmd: &str) {
        let _ = run_command(&["bash", "-c", cmd]);
    }

    pub fn install(&mut self) {
        self.print_installation_info();
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

        self.print_status(status);
        sleep(2);
    }

    fn print_status(&self, status: CommandExecutionStatus) {
        match status {
            CommandExecutionStatus::SUCCEEDED => {
                let message = format!(
                    "{} {}",
                    self.tool.yellow().bold(),
                    "installation complete".green()
                );
                logger::success(&message);
            }
            CommandExecutionStatus::FAILED => {
                let message = format!(
                    "{} {}",
                    self.tool.yellow().bold(),
                    "installation failed".red()
                );
                logger::error(&message);
            }
        }
    }

    pub fn update_shell(&mut self, config: String, anchor: &str) {
        match update_shell_configs(&config, anchor) {
            Ok(msg) => logger::success(&format!("{} {}", self.tool.purple().bold(), msg)),
            Err(err) => logger::warning(&format!(
                "Failed to add {} config to `.zshrc|.bashrc`: {}",
                self.tool.yellow().bold(),
                err
            )),
        }
    }
}
