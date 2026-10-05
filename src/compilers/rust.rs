use crate::utils::cmd::DevTool;
use crate::utils::logger;
use crate::utils::shell::update_shell_configs;
use colored::Colorize;

pub fn install_rust() {
    let tool = "Rust".to_string();
    let executor = "curl".to_string();
    let description = "toolchain".to_string();
    let is_sudo = false;
    let mut dev_tool = DevTool::new(tool.clone(), executor, description, is_sudo);

    let cmd = "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y";
    dev_tool.run(cmd);

    // Add ~/.local/bin to PATH so pip/virtualenv binaries are accessible
    let anchor = "Rust";
    let config_lines = [
        "",                            // empty line
        "# Rust -> Cargo",             // Toolchain ID
        "source \"$HOME/.cargo/env\"", // config
    ];
    let config = config_lines.join("\n");

    match update_shell_configs(&config, anchor) {
        Ok(msg) => logger::success(&format!("{} {}", tool.yellow().bold(), msg)),
        Err(err) => logger::warning(&format!(
            "Failed to add {} to shell config: {}",
            tool.yellow().bold(),
            err
        )),
    }

    dev_tool.check("rustc --version");
    dev_tool.check("cargo --version");
}
