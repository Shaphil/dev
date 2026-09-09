use crate::utils;
use crate::utils::CommandStatus;
use colored::Colorize;

struct DevTool {
    tool: String,
    installer: String,
    description: String,
    is_sudo: bool,
}

impl DevTool {
    fn new(tool: String, installer: String, description: String, is_sudo: bool) -> Self {
        Self {
            tool,
            installer,
            description,
            is_sudo,
        }
    }

    fn install(&mut self) {
        println!(
            "{} {} {}",
            "Installing".blue(),
            self.tool.blue().bold(),
            self.description.blue()
        );
        let status;
        if self.is_sudo {
            status = utils::run_command(&[
                "sudo",
                &self.installer.to_string(),
                "-S",
                &self.tool.to_string(),
            ]);
        } else {
            status =
                utils::run_command(&[&self.installer.to_string(), "-S", &self.tool.to_string()]);
        }
        print_status(&self.tool.to_string(), status);
        utils::sleep(2);
    }
}

pub fn install_nerd_fonts() {
    // install package manager `yay`
    let mut tool: String = "yay".to_string();
    let mut installer: String = "pacman".to_string();
    let mut description: String = "package manager...".to_string();
    let mut is_sudo = true;
    let mut dev_tool = DevTool::new(tool, installer, description, is_sudo);
    dev_tool.install();

    // install all nerd-fonts with `yay`
    tool = "nerd-fonts-complete".to_string();
    installer = "yay".to_string();
    description = "fonts pack".to_string();
    is_sudo = false;
    dev_tool = DevTool::new(tool, installer, description, is_sudo);
    dev_tool.install();
}

fn print_status(tool: &str, status: CommandStatus) {
    match status {
        CommandStatus::SUCCEEDED => {
            println!(
                "{} {}",
                tool.green().bold(),
                "installation complete".green()
            )
        }
        CommandStatus::FAILED => {
            println!("{} {}", tool.green().bold(), "installation failed".red())
        }
    }
}
