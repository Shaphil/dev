use crate::utils::cmd::DevTool;

pub fn install_dotnet() {
    let tool = "DotNET Core".to_string();
    let description = "SDK (LTS)".to_string();

    let cmd = "\
        mkdir -p ~/.dotnet && \
        curl -sSL https://dot.net/v1/dotnet-install.sh -o dotnet-install.sh && \
        chmod +x dotnet-install.sh && \
        ./dotnet-install.sh --channel LTS --install-dir ~/.dotnet && \
        rm dotnet-install.sh";

    let mut dev_tool = DevTool::new(tool.clone(), "sh -c".to_string(), description, false);
    dev_tool.run(cmd);

    let anchor = ".NET";
    let config = [
        "",
        "# .NET Environment Variables",
        "export DOTNET_ROOT=\"$HOME/.dotnet\"",
        "export PATH=\"$DOTNET_ROOT:$DOTNET_ROOT/tools:$PATH\"",
    ]
    .join("\n");
    dev_tool.update_shell(config, anchor);
    dev_tool.check("dotnet --version");
}
