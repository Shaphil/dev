use crate::utils::cmd::DevTool;
use crate::utils::logger;
use crate::utils::shell::update_shell_configs;
use colored::Colorize;

pub fn install_lsd() {
    let tool = "LSD".to_string();
    let executor = "".to_string();
    let description = "LSDeluxe".to_string();
    let is_root = false;
    let cmd = "cargo install lsd";
    let mut devtool = DevTool::new(tool.clone(), executor, description, is_root);
    devtool.run(cmd);

    let anchor = "lsd";
    let config_lines = ["\n", "alias la='lsd -la'", "alias ll='lsd -l'"];
    let config = config_lines.join("\n");

    match update_shell_configs(&config, anchor) {
        Ok(msg) => logger::success(&format!("{} {}", tool.yellow().bold(), msg)),
        Err(err) => logger::warning(&format!(
            "Failed to add {} to `.zshrc|.bashrc`: {}",
            tool, err
        )),
    }
}
