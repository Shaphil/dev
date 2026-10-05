# dev - Developer Essentials Installer

`dev` is a CLI tool designed to make it easy to set up a developer's machine with the tools
necessary to build their next big project.

## What's included

Currently, `dev` comes with setup for the following dev tools for the Linux environment,

### Category - Compilers/Interpreters/Runtimes

* Dotnet Core
* Golang
* Temurin JDK
* JavaFX
* NodeJS
* Yarn
* Pip
* Virtualenv
* uv
* Rust

### Category - DevOps

* Docker

### Category - Other

* Nerd Fonts
* OhMyPosh
* LSD

**Note -** All the tools mentioned are for the Linux (x86/x64) platform. Please report
any [issues](https://github.com/Shaphil/dev/issues) that you encounter.

Any ideas are welcome. That's what the [discussions](https://github.com/Shaphil/dev/discussions) are for.

## Usage

Ideally, you'll put `dev` in a directory and add that to your `PATH`, so that `dev` is available
to you in the terminal from any location. This could be a custom `~/bin` directory or any other
directory of your liking.

To get the installer, build the project with `cargo`, preferably a `release` build, like this,

```bash
cargo build --release
```

You'll find the binary in the `dev/target/release` directory. Copy `dev` from there to whichever
place you want to keep it in.

Invoking `dev` from the terminal without any commands/flags or with the `--help` flag will produce
the following output,

```bash
Usage: dev <COMMAND>

Commands:
  install  Install development tools. Use `dev install --help` for available options
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

From there, you can type in `dev install --help` to check the tools available to you for installation.
This is the output of running `dev install --help` in a terminal,

```bash
Install development tools. Use `dev install --help` for available options

Usage: dev install <TOOLS>...

Arguments:
  <TOOLS>...
          The tool to install

          Possible values:
          - python-tools: Install Python Tools (pip, virtualenv)
          - uv:           Install uv
          - go:           Install Go
          - jdk:          Install JDK
          - openjfx:      Install OpenJFX
          - dotnet:       Install dotnet-sdk
          - nodejs:       Install NodeJS and npm
          - yarn:         Install yarn
          - rust:         Install Rust
          - docker:       Install Docker
          - nerdfonts:    Install Nerd Fonts
          - oh-my-posh:   Install oh-my-posh
          - lsd:          Install lsd
          - all:          Install all tools

Options:
  -h, --help
          Print help (see a summary with '-h')
```

## Breaking change

* Previously to install `pip`, you needed this `dev install --pip`
* Now, you can install `pip` with `dev install pip`
* You can still chain commands for installing tools like before with a slight change,
    * before: `dev install --pip --go`
    * now: `dev install pip go`

Please note that, if `dev` needs to download something (a tarball for instance),
it will download that from the directory/location from which it was invoked.
Once it's done with the installation, it will remove/delete up the installer. 
