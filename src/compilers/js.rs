use crate::utils::cmd::DevTool;
use crate::utils::shell::update_shell_configs;
use crate::utils::{cmd, logger};
use colored::Colorize;
use std::{thread, time};

pub fn install_nodejs() {
    let tool = "nodejs".to_string();
    let description = "Node.js LTS Runtime".to_string();

    // Pull the latest LTS version dynamically from the Nodejs metadata index
    let cmd = "\
        mkdir -p ~/.local/opt && \
        NODE_VERSION=$(curl -s https://nodejs.org/dist/index.json | jq -r '[.[] | select(.lts != false)][0].version') && \
        echo \"Downloading Node.js version: $NODE_VERSION\" && \
        curl -sL https://nodejs.org/dist/$NODE_VERSION/node-$NODE_VERSION-linux-x64.tar.xz -o node.tar.xz && \
        tar -xf node.tar.xz -C ~/.local/opt/ && \
        rm -rf ~/.local/opt/node && \
        mv ~/.local/opt/node-$NODE_VERSION-linux-x64 ~/.local/opt/node && \
        rm node.tar.xz";

    let mut dev_tool = DevTool::new(tool.clone(), "sh -c".to_string(), description, false);
    dev_tool.run(cmd);

    let anchor = "NODE_HOME";
    let config = [
        "",
        "# Node.js Environment Variables",
        "export NODE_HOME=\"$HOME/.local/opt/node\"",
        "export PATH=\"$NODE_HOME/bin:$PATH\"",
    ]
    .join("\n");

    match update_shell_configs(&config, anchor) {
        Ok(msg) => logger::success(&format!("{} {}", tool.yellow().bold(), msg)),
        Err(err) => logger::warning(&format!(
            "Failed to add {} config to `.zshrc|.bashrc`: {}",
            tool.yellow().bold(),
            err
        )),
    }
}

pub fn install_yarn() {
    println!("{}", "Installing yarn...".blue());
    cmd::run_command(&["npm", "config", "set", "prefix", "~/.npm-global"]);
    cmd::run_command(&["npm", "install", "--global", "yarn"]);
    cmd::run_command(&["yarn", "--version"]);
    println!("{}", "yarn installation complete".blue());
    thread::sleep(time::Duration::from_secs(2));
}
