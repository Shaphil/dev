use crate::utils::cmd::DevTool;
use crate::utils::logger;
use crate::utils::shell::update_shell_configs;
use colored::Colorize;

pub fn install_golang() {
    let tool = "Go".to_string();
    let description = "programming language".to_string();

    // Fetches the latest Go version dynamically from the official API
    let cmd = "\
        mkdir -p ~/.local/opt ~/.go/bin && \
        GO_VERSION=$(curl -s https://go.dev/VERSION?m=text | head -n 1) && \
        echo \"Downloading Go version: $GO_VERSION\" && \
        curl -sL https://golang.org/dl/$GO_VERSION.linux-amd64.tar.gz -o go.tar.gz && \
        rm -rf ~/.local/opt/go && \
        tar -C ~/.local/opt -xzf go.tar.gz && \
        rm go.tar.gz";

    let mut dev_tool = DevTool::new(tool.clone(), "sh -c".to_string(), description, false);
    dev_tool.run(cmd);

    let anchor = "GOLANG";
    let config = [
        "",
        "# Go Environment Variables",
        "export PATH=\"$HOME/.local/opt/go/bin:$PATH\"",
        "export GOPATH=\"$HOME/go\"",
        "export PATH=\"$GOPATH/bin:$PATH\"",
    ]
    .join("\n");

    match update_shell_configs(&config, anchor) {
        Ok(msg) => logger::success(&format!("{} {}", tool.cyan().bold(), msg)),
        Err(err) => logger::warning(&format!(
            "Failed to add {} to `.zshrc|.bashrc`: {}",
            tool, err
        )),
    }
}
