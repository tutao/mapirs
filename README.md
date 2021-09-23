# Mapirs

This is a small, windows-only dynamic library that can be used to translate Windows MAPI calls to command line
arguments.

The main purpose is to enable Tutanota Desktop on Windows to handle requests to send files via mail, like they can be
issued via the File Explorer context menu or in programs like MS Office and LibreOffice.

# Build

## Compile on windows

### Prerequisites

* Microsoft Visual C++ 2019 build tools
* cargo

### Commands

`cargo build` or
`cargo build --release`

the dll will be placed in ./target/{release,debug}/mapirs.dll

## Crosscompile on Linux

.cargo/config is set up so "cargo build" will use the x86_64-pc-windows-gnu target. Running tests requires wine to be
installed.

### Prerequisites

* package gcc-mingw-w64
* cargo
* wine if running tests

### Commands

`cargo build` or
`cargo build --release`

the dll will be placed in ./target/x86_64-pc-windows-gnu/{release,debug}/mapirs.dll

# Notes

To get the IntelliJ IDEA Rust Plugin to ignore/check code that is inactive due to attributes like
`#[cfg(target_os = "windows")]`, open `Ctrl + Shift + A -> Registry... ` and toggle `org.rust.lang.cfg.attributes`