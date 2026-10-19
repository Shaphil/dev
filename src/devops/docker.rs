use crate::utils::cmd::DevTool;
use crate::utils::logger;

pub fn install_docker() {
    // Download and extract the official static Docker CLI binary
    let mut dev_tool = DevTool::new(
        "Docker CLI".to_string(),
        "sh -c".to_string(),
        "Docker static CLI binary".to_string(),
        false, // is_sudo = false for downloading and untarring locally
    );

    // Dynamically fetch the latest .tgz filename from Docker's index server via a shell command output
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(
            r#"curl -s https://download.docker.com/linux/static/stable/x86_64/ |
            grep -oE 'docker-[0-9]+\.[0-9]+\.[0-9]+\.tgz' |
            tail -n 1"#,
        )
        .output();

    let mut package = match output {
        Ok(out) if !out.stdout.is_empty() => {
            String::from_utf8_lossy(&out.stdout).trim().to_string()
        }
        _ => {
            logger::warning("Failed to fetch latest Docker version, falling back to default.");
            "docker-29.8.2.tgz".to_string()
        }
    };

    logger::info(&format!("Latest Docker CLI package: {}", package));

    let mut cmd = format!(
        "curl -fsSL https://download.docker.com/linux/static/stable/x86_64/{} |\
         tar xz docker/docker --strip-components=1",
        package
    );
    dev_tool.run(&cmd);

    // Copy the extracted binary to /usr/local/bin (requires root)
    dev_tool.is_sudo = true;
    dev_tool.run("cp -rp ./docker /usr/local/bin/ && rm -f ./docker");

    // Download Docker Desktop (.pkg.tar.zst)
    package = "docker-desktop-x86_64.pkg.tar.zst".to_string();
    let download_pkg = format!(
        "curl -O https://desktop.docker.com/linux/main/amd64/{}",
        package
    );
    dev_tool.is_sudo = false;
    dev_tool.run(&download_pkg);

    // Install Docker Desktop
    dev_tool.is_sudo = true;
    cmd = format!("pacman -U --noconfirm ./{}", package);
    dev_tool.run(&cmd);
    dev_tool.cleanup(package.as_str());
}
