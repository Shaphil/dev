use crate::utils::cmd::DevTool;
use crate::utils::logger;
use crate::utils::shell::update_shell_configs;
use std::fs;

pub fn install_oh_my_posh() {
    let tool = "ohMyPosh".to_string();
    let executor = "curl -s".to_string();
    let description = "prompt engine".to_string();
    let is_sudo = false;

    let mut dev_tool = DevTool::new(tool.clone(), executor, description, is_sudo);
    let cmd = "curl -s https://ohmyposh.dev/install.sh | bash -s -- -d ~/bin";
    dev_tool.run(cmd);

    // Disable Manjaro's default system-level powerlevel10k prompt (requires sudo)
    disable_manjaro_p10k_system();

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

/// Safely comment out the p10k source line in the global Manjaro prompt config using sed
fn disable_manjaro_p10k_system() {
    let mut patch_tool = DevTool::new(
        "manjaro-p10k-patch".to_string(),
        "".to_string(),
        "Disabling default Manjaro powerlevel10k theme".to_string(),
        true, // Requires sudo!
    );

    // This sed command safely inserts a '#' in front of the source line if it isn't already commented.
    // It uses an alternate delimiter (|) in sed to avoid path escaping issues.
    let patch_cmd = "\
        if grep -q '^[^#]*/usr/share/zsh-theme-powerlevel10k/powerlevel10k.zsh-theme' /usr/share/zsh/manjaro-zsh-prompt; then \
            sed -i 's|^\\s*source /usr/share/zsh-theme-powerlevel10k/powerlevel10k.zsh-theme|# &|' /usr/share/zsh/manjaro-zsh-prompt; \
        fi";

    patch_tool.run(patch_cmd);
}
