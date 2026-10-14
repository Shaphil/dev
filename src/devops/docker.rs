use crate::utils::cmd::DevTool;

pub fn install_docker() {
    let tool = "Docker".to_string();
    let description = "containerization tool".to_string();
    let executor = "pacman".to_string();
    let is_sudo = false;
    let package = "docker-desktop-x86_64.pkg.tar.zst";

    let mut dev_tool = DevTool::new(tool, executor, description, is_sudo);
    let mut cmd = format!(
        "curl -O https://desktop.docker.com/linux/main/amd64/{}",
        package
    );
    dev_tool.run(&cmd);

    dev_tool.is_sudo = true;
    cmd = format!("pacman -U --noconfirm ./{}", package);
    dev_tool.run(&cmd);
    dev_tool.check("pacman -Q docker-desktop");
}
