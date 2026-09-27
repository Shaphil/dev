use crate::utils::cmd::DevTool;
use crate::utils::logger;
use crate::utils::shell::update_shell_configs;
use colored::Colorize;

pub fn install_dotnet() {
    let tool = "DotNET".to_string();
    let description = "SDK (LTS)".to_string();

    let cmd = "\
        mkdir -p ~/.dotnet && \
        curl -sSL https://dot.net/v1/dotnet-install.sh -o dotnet-install.sh && \
        chmod +x dotnet-install.sh && \
        ./dotnet-install.sh --channel LTS --install-dir ~/.dotnet && \
        rm dotnet-install.sh";

    let mut dev_tool = DevTool::new(tool.clone(), "sh -c".to_string(), description, false);
    dev_tool.run(cmd);

    let anchor = "DOTNET_ROOT";
    let config = [
        "\n",
        "# .NET Environment Variables",
        "export DOTNET_ROOT=\"$HOME/.dotnet\"",
        "export PATH=\"$DOTNET_ROOT:$DOTNET_ROOT/tools:$PATH\"",
    ]
    .join("\n");

    match update_shell_configs(&config, anchor) {
        Ok(msg) => logger::success(&format!("{} {}", tool.purple().bold(), msg)),
        Err(err) => logger::warning(&format!(
            "Failed to add {} config to `.zshrc|.bashrc`: {}",
            tool.yellow().bold(),
            err
        )),
    }
}
