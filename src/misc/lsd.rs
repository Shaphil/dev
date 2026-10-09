use crate::utils::cmd::DevTool;

pub fn install_lsd() {
    let tool = "LSD".to_string();
    let executor = "".to_string();
    let description = "LSDeluxe".to_string();
    let is_root = false;
    let cmd = "cargo install lsd";
    let mut devtool = DevTool::new(tool.clone(), executor, description, is_root);
    devtool.run(cmd);

    let anchor = "LSD";
    let config_lines = [
        "",
        "# LSD - ls deluxe",
        "alias la='lsd -la'",
        "alias ll='lsd -l'",
    ];
    let config = config_lines.join("\n");
    devtool.update_shell(config, anchor);
}
