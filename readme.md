# dev - Developer Essentials Installer

`dev` is a CLI tool designed to make it easy to set up a developer's machine with the tools necessary to build their
next big project.

## What's included

Currently, `dev` comes with setup for the following dev tools for the Linux environment,

### Category - Compilers/Interpreters/Runtimes

* Dotnet Core
* Golang
* Oracle JDK
* JavaFX
* NodeJS
* Yarn
* Pip
* Virtualenv
* Rust

### Category - DevOps

* Docker

**Note -** All the tools mentioned are for the Linux (x86/x64) platform, more specifically, Manjaro Linux. The installer
was developed on Manjaro, so some installation uses Manjaro specific installation (for instance, installation with
`pacman` package manager). This in theory should also allow the installer to run seamlessly in any Arch based distro as
well, though the installer was not tested on other distros. So, if you use Arch or any other Arch based distro, you are
welcome to test it. Please report any [issues](https://github.com/Shaphil/dev/issues) that you encounter. Support for
other flavors of Linux coming soon.

Any ideas are welcome. That's what the [discussions](https://github.com/Shaphil/dev/discussions) are for.

## Usage

Ideally, you'll put `dev` in a directory and add that to your `PATH`, so that `dev` is available to you in the terminal
from any location. This could be a custom `~/bin` directory or any other directory of your liking.

To get the installer, build the project with `cargo`, preferably a `release` build, like this,

```bash
cargo build --release
```

You'll find the binary in the `dev/target/release` directory. Copy `dev` from there to whichever place you want to keep
it.

Invoking `dev` from the terminal without any commands/flags or with the `--help` flag will produce the following output,

```bash
Usage: dev <COMMAND>

Commands:
  install  Install development tools. Use `dev install --help` for available options
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

From there, you can type in `dev install --help` to check the tools available to you for installation. This is the
output of running `dev install --help` in a terminal,

```bash
Install development tools. Use `dev install --help` for available options

Usage: dev install [OPTIONS]

Options:
      --pip         Install pip
      --virtualenv  Install virtualenv
      --go          Install Go
      --jdk         Install JDK
      --openjfx     Install OpenJFX
      --dotnet      Install dotnet-sdk
      --nodejs      Install NodeJS and npm
      --yarn        Install yarn
      --rust        Install Rust
      --docker      Install Docker
      --all         Install all tools
  -h, --help        Print help
```

For installing a particular tool, you'll pass the related flag to `dev install`. To install `pip` for example, you'll
type `dev install --pip`, and to install all the tools you can use the `--all` flag, like `dev install --all`. You can
also chain multiple flags together, like, `dev install --pip --go` to install `pip` and `go`.

Please note that, if `dev` needs to download something (a tarball for instance), it will download that from the
directory/location from which it was invoked. It will not automatically clean up the leftover archives after
installation, this can be a feature that can be added in the future. This is why it is recommended that you put `dev`
inside a directory of your choosing for now. That way the leftovers from the installation will be inside one directory,
making clean up much easier and without leaving the filesystem messy. `dev` will also ask for the `sudo` users' password
for most installations as they are often installed under the root(`/`) directory (`/usr/local` for golang for instance).
This is done primarily due to recommendation (see: <https://go.dev/doc/install>) and ease of use. Although this too can
be an option during the installation, and may be added in the future as a feature.
