# Mapirs

This is a small, windows-only dynamic library that can be used to translate Windows MAPI calls to command line
arguments.

It handles calls to the functions defined in the mapi.h header, which is not available directly from Microsoft anymore.
You can get it from https://github.com/stephenegriffin/MAPIStubLibrary/blob/master/include/MAPI.h

The main purpose is to enable Tutanota Desktop on Windows to handle requests to send files via mail, like they can be
issued via the File Explorer context menu or in programs like MS Office and LibreOffice.

# Build

## Compile on windows

The project is set up to compile for the `x86_64-pc-windows-gnu` target by default, so if you're developing on windows,
you'll have to override `.cargo/config.toml` as shown in the **"Commands"** section below.

### Prerequisites

* Microsoft Visual C++ 2019 build tools
* cargo

### Commands

`cargo build --target "x86_64-pc-windows-msvc"`

or

`cargo build --release --target "x86_64-pc-windows-msvc"`

the dll will be placed in `./target/x86_64-pc-windows-msvc/{release,debug}/mapirs.dll`

## Crosscompile on Linux

The project is set up so "cargo build" will use the `x86_64-pc-windows-gnu` target. Running tests requires wine to be
installed.

### Prerequisites

* package gcc-mingw-w64
* cargo
* wine

### Commands

`cargo build`

or

`cargo build --release`

the dll will be placed in `./target/x86_64-pc-windows-gnu/{release,debug}/mapirs.dll`

# Notes

* To get the IntelliJ IDEA Rust Plugin to ignore/check code that is inactive due to attributes like
  `#[cfg(target_os = "windows")]`, open `Ctrl + Shift + A -> Registry... ` and toggle `org.rust.lang.cfg.attributes`
* licenses.html has been generated with https://github.com/EmbarkStudios/cargo-about :
  `cargo-about init` and then `cargo-about generate about.hbs > licenses.html`
* the CI script uses clippy and rustfmt to check the code, so run `cargo clippy` and `cargo fmt` before committing (in that
  order)
