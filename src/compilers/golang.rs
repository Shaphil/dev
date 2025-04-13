use std::process::Command;
use std::path::Path;
use std::{fs, thread, time};
use crate::utils;

pub fn install_go() {
    println!("Installing Latest Go...");

    // Fetch the latest Go version
    let version_output = Command::new("curl")
        .args(&["-s", "https://go.dev/VERSION?m=text"])
        .output()
        .expect("Failed to get Go version");

    let version_string = String::from_utf8_lossy(&version_output.stdout);

    // Extract only the first line, which contains the version number
    let version = version_string
        .lines()
        .next()
        .unwrap_or("")
        .trim()
        .to_string();

    if !version.starts_with("go") {
        panic!("Invalid Go version string received: {}", version);
    }

    let filename = format!("{}.linux-amd64.tar.gz", version);
    let url = format!("https://dl.google.com/go/{}", filename);

    println!("Downloading Go version: {} from {}", version, url);

    // Download the tarball
    utils::run_command(&["curl", "-L", "-o", &filename, &url]);

    // Remove existing Go installation
    if Path::new("/usr/local/go").exists() {
        utils::run_command(&["sudo", "rm", "-rf", "/usr/local/go"]);
    }

    // Extract Go tarball
    utils::run_command(&["sudo", "tar", "-C", "/usr/local", "-xzf", &filename]);

    // Clean up tarball
    if Path::new(&filename).exists() {
        fs::remove_file(&filename).expect("Failed to remove tarball");
    }

    // Set up Go environment variables
    let profile_path = "/etc/profile.d/go.sh";
    let go_path_export = "export PATH=$PATH:/usr/local/go/bin";

    if !Path::new(profile_path).exists() {
        utils::run_command(&[
            "sudo",
            "bash",
            "-c",
            &format!("echo '{}' > {}", go_path_export, profile_path),
        ]);
        utils::run_command(&["sudo", "chmod", "+x", profile_path]);
    }

    // Source the profile
    utils::run_command(&["bash", "-c", "source /etc/profile.d/go.sh"]);

    // Verify the installation
    utils::run_command(&["go", "version"]);

    println!("Go installation complete!");
    thread::sleep(time::Duration::from_secs(2));
}