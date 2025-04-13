use std::process::Command;

pub fn run_command(command: &[&str]) {
    let mut cmd = Command::new(command[0]);
    cmd.args(&command[1..]);
    match cmd.status() {
        Ok(status) if status.success() => {
            if command[0] != "curl" && command[0] != "sudo" {
                let output = cmd.output().expect("failed to execute command");
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
        }
        Ok(status) => {
            eprintln!("Command failed: {}", status);
        }
        Err(e) => {
            eprintln!("Error running command: {}", e);
        }
    }
}