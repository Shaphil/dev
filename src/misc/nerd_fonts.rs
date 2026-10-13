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

    if is_all {
        logger::info("Preparing to download the full Nerd Fonts suite (~10GB equivalent)...");

        // 1. Create directory
        let mut setup_tool = DevTool::new(
            "NerdFonts-Setup".to_string(),
            "sh -c".to_string(),
            "Nerd Fonts".to_string(),
            false,
        );
        setup_tool.run("mkdir -p ~/.local/share/fonts/NerdFonts");

        // 2. Loop through all fonts with live logging
        for (i, font) in AVAILABLE_FONTS.iter().enumerate() {
            logger::info(&format!(
                "[{}/{}] Downloading {}...",
                i + 1,
                AVAILABLE_FONTS.len(),
                font
            ));

            let zip_name = format!("{}.zip", font);
            let url = format!(
                "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/{}",
                zip_name
            );

            let font_cmd = format!(
                "curl -sL {} -o {} && unzip -o {} -d ~/.local/share/fonts/NerdFonts/ && rm {}",
                url, zip_name, zip_name, zip_name
            );

            let mut font_tool = DevTool::new(
                font.to_string(),
                "sh -c".to_string(),
                "Nerd Fonts".to_string(),
                false,
            );
            font_tool.run(&font_cmd);
        }

        // 3. Refresh font cache
        logger::info("Updating font cache...");
        let mut cache_tool = DevTool::new(
            "NerdFonts-Cache".to_string(),
            "sh -c".to_string(),
            "Nerd Fonts".to_string(),
            false,
        );
        cache_tool.run("fc-cache -f");

        logger::info("Full Nerd Fonts suite installed successfully!");
    } else {
        // Single font installation path (keeps the clean single-cmd pattern)
        let zip_name = format!("{}.zip", selected_font);
        let download_url = format!(
            "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/{}",
            zip_name
        );

        let cmd = format!(
            "mkdir -p ~/.local/share/fonts/NerdFonts && \
             curl -sL {} -o {} && \
             unzip -o {} -d ~/.local/share/fonts/NerdFonts/ && \
             rm {} && \
             fc-cache -f",
            download_url, zip_name, zip_name, zip_name
        );

        let mut dev_tool = DevTool::new(
            selected_font.clone(),
            "sh -c".to_string(),
            "Nerd Fonts".to_string(),
            false,
        );
        dev_tool.run(&cmd);

        logger::info(&format!(
            "Nerd Font '{}' installed successfully!",
            selected_font
        ));
    }
}
