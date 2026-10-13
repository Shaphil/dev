use super::fonts_list::AVAILABLE_FONTS;
use crate::utils::cmd::DevTool;
use crate::utils::logger;
use std::io::{self, Write};

pub fn install_nerd_fonts(unattended: bool) {
    let selected_font: String;
    let is_all;

    let github_token = std::env::var("GH_TOKEN")
        .unwrap_or_else(|_| "environment variable `GH_TOKEN` is not set or missing".to_string());

    if github_token.is_empty() {
        logger::warning("`GH_TOKEN` not found. Installing all Nerd Fonts will fail.");
    }

    if unattended {
        // In unattended mode, installing `CascadiaCode` as the default
        selected_font = AVAILABLE_FONTS[11].to_string();
        is_all = false;
        logger::info(&format!(
            "(unattended mode) Auto-selecting Nerd Font: {}",
            selected_font
        ));
    } else {
        logger::info("Available Nerd Fonts:");
        for (index, font) in AVAILABLE_FONTS.iter().enumerate() {
            println!("{}. {}", index + 1, font);
        }

        loop {
            print!("Please choose a font to install (0 for all): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            let input = input.trim();
            if let Ok(choice) = input.parse::<usize>() {
                if choice == 0 {
                    is_all = true;
                    selected_font = "all".to_string();
                    break;
                } else if (1..=AVAILABLE_FONTS.len()).contains(&choice) {
                    is_all = false;
                    selected_font = AVAILABLE_FONTS[choice - 1].to_string();
                    break;
                }
            }
            logger::error("Invalid choice.");
        }
    }

    let cmd = if is_all {
        logger::info("Preparing to download the full Nerd Fonts suite (~10GB)...");
        format!(
            "export GH_TOKEN={} && \
             curl -s https://raw.githubusercontent.com/ryanoasis/nerd-fonts/master/install.sh -o install.sh && \
             chmod u+x install.sh && \
             ./install.sh install all && \
             rm install.sh",
            github_token
        )
    } else {
        // Single font installation from direct GitHub releases
        let zip_name = format!("{}.zip", selected_font);
        let download_url = format!(
            "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/{}",
            zip_name
        );
        format!(
            "mkdir -p ~/.local/share/fonts/NerdFonts && \
             curl -sL {} -o {} && \
             unzip -o {} -d ~/.local/share/fonts/NerdFonts/ && \
             rm {} && \
             fc-cache -f",
            download_url, zip_name, zip_name, zip_name
        )
    };

    let tool_name = if is_all {
        "All".to_string()
    } else {
        format!("{selected_font}")
    };
    let mut dev_tool = DevTool::new(
        tool_name,
        "sh -c".to_string(),
        "Nerd Fonts".to_string(),
        false,
    );
    dev_tool.run(&cmd);
}
