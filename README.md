# **nvim-updater-rs** ~ A Neovim command-line updater

[![crates.io](https://img.shields.io/crates/v/nvim-updater-rs)](https://crates.io/crates/nvim-updater-rs)
[![license](https://img.shields.io/crates/l/nvim-updater-rs)](https://github.com/olacin/nvim-updater-rs/blob/main/LICENSE)

## Description

If you've spent any amount of time in neovim and its configuration you probably are updating it often to get the latest features.

Updating should be quick & easy so you can run that everyday in an automated (or not !) manner.

## Installation

From **crates.io**: `cargo install nvim-updater-rs`

## Usage

Use `nvim-updater-rs --help` to display help on commandline.
```
nvim-updater-rs 0.1.0
Nicolas Picard
A Neovim command-line updater.

USAGE:
    nvim-updater-rs [OPTIONS]

OPTIONS:
    -c, --check                        Check only if a new version is available
    -d, --destination <DESTINATION>    Executable directory destination [default: /usr/bin/nvim]
    -h, --help                         Print help information
    -V, --version                      Print version information
```

## Example outputs

### Check

```
▶ nvim-updater-rs --check
Gathering information on versions
✅ Already at the latest version: latest=8952def50 current=8952def50
```

### Custom destination

By default output of executable is `/usr/bin/nvim`. Depending on your environment you may not have permissions to write in this directory. You can override this option by providing the `-d` or `--destination` option.

```
▶ nvim-updater-rs -d
Gathering information on versions
✅ Already at the latest version: latest=8952def50 current=8952def50
```
