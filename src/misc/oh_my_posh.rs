use crate::utils::DevTool;

pub fn install_oh_my_posh() {
    // curl -s https://ohmyposh.dev/install.sh | bash -s -- -d ~/bin
    // tool: String, executor: String, description: String, is_sudo: bool
    let tool = "ohMyPosh".to_string();
    let executor = "curl -s".to_string().to_string();
    let description = "prompt engine".to_string();
    let is_sudo = false;

    let mut dev_tool = DevTool::new(tool, executor, description, is_sudo);
    let cmd = "curl -s https://ohmyposh.dev/install.sh | bash -s -- -d ~/bin";
    dev_tool.custom_install(cmd);
}
