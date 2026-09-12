use crate::utils::cmd::DevTool;

pub fn install_nerd_fonts() {
    // install package manager `yay`
    let mut tool: String = "yay".to_string();
    let mut executor: String = "pacman".to_string();
    let mut description: String = "package manager...".to_string();
    let mut is_sudo = true;
    let mut dev_tool = DevTool::new(tool, executor, description, is_sudo);
    dev_tool.install();

    // install all nerd-fonts with `yay`
    tool = "nerd-fonts-complete".to_string();
    executor = "yay".to_string();
    description = "fonts pack".to_string();
    is_sudo = false;
    dev_tool = DevTool::new(tool, executor, description, is_sudo);
    dev_tool.install();
}
