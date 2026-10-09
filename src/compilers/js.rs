use crate::utils::cmd::{CommandExecutionStatus, DevTool};
use crate::utils::{cmd, logger};

pub fn install_nodejs() {
    let tool = "Node.js LTS".to_string();
    let description = "Runtime".to_string();

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

    let anchor = "Node.js";
    let config = [
        "",
        "# Node.js Environment Variables",
        "export NODE_HOME=\"$HOME/.local/opt/node\"",
        "export PATH=\"$NODE_HOME/bin:$PATH\"",
    ]
    .join("\n");
    dev_tool.update_shell(config, anchor);
    dev_tool.check("node -v");
    dev_tool.check("npm -v");
}

pub fn install_yarn() {
    logger::info("Checking for npm installation...");
    let status = cmd::run_command(&["npm", "-v"]);
    match status {
        CommandExecutionStatus::SUCCEEDED => {
            let tool = "yarn".to_string();
            let description = "package manager via npm".to_string();
            let is_sudo = false;

            let mut dev_tool = DevTool::new(tool.clone(), "".to_string(), description, is_sudo);

            // Commands to install `yarn` via `npm`
            let cmd = "\
                npm config set prefix '~/.npm-global' && \
                npm install --global yarn && \
                yarn --version";

            dev_tool.run(cmd);

            // Add `~/.npm-global/bin` to PATH
            let anchor = "NPM";
            let config_lines = [
                "",
                "# NPM Global Binaries & Yarn",
                "export PATH=\"$HOME/.npm-global/bin:$PATH\"",
            ];
            let config = config_lines.join("\n");
            dev_tool.update_shell(config, anchor);
            dev_tool.check("yarn -v");
        }
        CommandExecutionStatus::FAILED => {
            logger::error("npm not found. Installing Node & npm");
            install_nodejs();
        }
    }
}
