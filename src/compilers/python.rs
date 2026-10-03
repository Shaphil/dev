use colored::Colorize;

use crate::utils::cmd::DevTool;
use crate::utils::logger;
use crate::utils::shell::update_shell_configs;

pub fn install_python_tools() {
    let tool = "Python Tools".to_string();
    let description = "User-space pip and virtualenv".to_string();
    let executor = "sh -c".to_string();
    let is_sudo = false;

    // Use ensurepip first, fallback to get-pip.py if stripped by distro, then install virtualenv
    let cmd = "\
    curl -sS https://bootstrap.pypa.io/get-pip.py -o get-pip.py && \
    python get-pip.py --user --break-system-packages && \
    rm get-pip.py && \
    python -m pip install --user --break-system-packages --upgrade pip && \
    python -m pip install --user --break-system-packages virtualenv";

    let mut dev_tool = DevTool::new(tool.clone(), executor, description, is_sudo);
    dev_tool.run(cmd);

    // Add ~/.local/bin to PATH so pip/virtualenv binaries are accessible
    let anchor = "Python Tools";
    let config_lines = [
        "",
        "# Python Tools (pip, virtualenv)",
        "export PATH=\"$HOME/.local/bin:$PATH\"",
    ];
    let config = config_lines.join("\n");

    match update_shell_configs(&config, anchor) {
        Ok(msg) => logger::success(&format!("{} {}", tool.yellow().bold(), msg)),
        Err(err) => logger::warning(&format!(
            "Failed to add Python local bin to shell config: {}",
            err
        )),
    }

    // 3. Verify installations
    dev_tool.check("pip --version");
    dev_tool.check("virtualenv --version");
}

pub fn install_uv() {
    let tool = "uv".to_string();
    let executor = "".to_string();
    let description =
        "An extremely fast Python package and project manager, written in Rust.".to_string();
    let is_sudo = false;
    let mut cmd = "curl -LsSf https://astral.sh/uv/install.sh | sh";

    let mut dev_tool = DevTool::new(tool, executor, description, is_sudo);
    dev_tool.run(cmd);

    cmd = "uv --version";
    dev_tool.check(cmd);
}
