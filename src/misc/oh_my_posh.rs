use crate::utils::cmd::DevTool;
use crate::utils::logger;
use crate::utils::shell::update_shell_configs;

pub fn install_oh_my_posh() {
    let tool = "ohMyPosh".to_string();
    let executor = "curl -s".to_string().to_string();
    let description = "prompt engine".to_string();
    let is_sudo = false;

    let mut dev_tool = DevTool::new(tool.clone(), executor, description, is_sudo);
    let cmd = "curl -s https://ohmyposh.dev/install.sh | bash -s -- -d ~/bin";
    dev_tool.custom_install(cmd);

    let anchor = "oh-my-posh init";
    let config_lines = [
        "# Oh-my-posh, with `jandedobbeleer` theme",
        "eval \"$(oh-my-posh init zsh --config ~/.cache/oh-my-posh/themes/jandedobbeleer.omp.json)\"",
    ];
    let config = config_lines.join("\n");

    match update_shell_configs(&config, anchor) {
        Ok(_) => logger::success(&format!("{} added to `.zshrc|.bashrc`", tool)),
        Err(err) => logger::warning(&format!(
            "Failed to add {} to `.zshrc|.bashrc`: {}",
            tool, err
        )),
    }
}
