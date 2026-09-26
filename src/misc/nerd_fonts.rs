use crate::utils::cmd::DevTool;
use crate::utils::logger;

pub fn install_nerd_fonts() {
    let github_token = std::env::var("GH_TOKEN")
        .unwrap_or_else(|_| "environment variable `GH_TOKEN` is not set or missing".to_string());

    if github_token.is_empty() {
        let message = format!(
            "{}",
            "GH_TOKEN not found. GitHub API rate \
            limits may cause Nerd Fonts installation to fail."
        );
        logger::warning(&message);
    }

    let tool = "Nerd Fonts".to_string();
    let executor = "pacman".to_string();
    let description = "fonts pack".to_string();
    let is_sudo = false;
    let cmd = format!(
        "export GH_TOKEN={} && \
        curl -s https://raw.githubusercontent.com/ryanoasis/nerd-fonts/master/install.sh -o install.sh && \
        chmod u+x install.sh && \
        ./install.sh install all && \
        rm install.sh",
        github_token
    );

    let mut dev_tool = DevTool::new(tool, executor, description, is_sudo);
    dev_tool.run(&cmd);
}
