use crate::utils;
use colored::Colorize;
use std::{thread, time};

// TODO: install from tarballs, e.g. `dotnet-sdk-9.0.203-linux-x64.tar.gz`
// TODO: available from `https://builds.dotnet.microsoft.com/dotnet/Sdk/9.0.203/dotnet-sdk-9.0.203-linux-x64.tar.gz`
// More: https://dotnet.microsoft.com/en-us/download/dotnet/8.0
// and, https://dotnet.microsoft.com/en-us/download/dotnet/9.0
pub fn install_dotnet() {
    println!("{}", "Installing dotnet-sdk...".blue());
    utils::run_command(&["sudo", "pamac", "install", "dotnet-sdk"]);
    utils::run_command(&["dotnet", "--version"]);
    println!("{}", "dotnet-sdk installation complete".blue());
    thread::sleep(time::Duration::from_secs(2));
}
