### Build

crosscompile on linux:

cargo build --target x86_64-pc-windows-gnu

### Notes

To get the IntelliJ IDEA Rust Plugin to ignore/check code that is inactive due to attributes like
`#[cfg(target_os = "windows")]`, open `Ctrl + Shift + A -> Registry... ` and toggle `org.rust.lang.cfg.attributes`