use crate::utils;
use crate::utils::CommandStatus;
use colored::Colorize;

pub fn install_nerd_fonts() {
    // install package manager `yay`
    let mut tool = "yay";
    println!(
        "{} {} {}",
        "Installing".blue(),
        tool.blue(),
        "package manager...".blue()
    );
    let status = utils::run_command(&["sudo", "pacman", "-S", "yay"]);
    print_status(tool, status);
    utils::sleep(2);

    // install all nerd-fonts with `yay`
    tool = "Nerd Fonts Complete";
    println!(
        "{} {} {}",
        "Installing".blue(),
        tool.blue().bold(),
        "...".blue()
    );
    let status = utils::run_command(&["yay", "-S", "nerd-fonts-complete"]);
    print_status(tool, status);
    utils::sleep(2);
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
