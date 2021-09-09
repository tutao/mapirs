### Build

debug crosscompile on linux (**requires package gcc-mingw-w64**):

`cargo build --target x86_64-pc-windows-gnu`

release crosscompile on linux (**requires package gcc-mingw-w64**):

`cargo build --target x86_64-pc-windows-gnu --release`

the dll will be placed in ./target/x86_64-pc-windows-gnu/{release,debug}/mapirs.dll

### Notes

To get the IntelliJ IDEA Rust Plugin to ignore/check code that is inactive due to attributes like
`#[cfg(target_os = "windows")]`, open `Ctrl + Shift + A -> Registry... ` and toggle `org.rust.lang.cfg.attributes`