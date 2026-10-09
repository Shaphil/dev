use crate::utils::cmd::DevTool;

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

    let anchor = "Go";
    let config = [
        "",
        "# Go Environment Variables",
        "export PATH=\"$HOME/.local/opt/go/bin:$PATH\"",
        "export GOPATH=\"$HOME/go\"",
        "export PATH=\"$GOPATH/bin:$PATH\"",
    ]
    .join("\n");
    dev_tool.update_shell(config, anchor);
    dev_tool.check("go version");
}
