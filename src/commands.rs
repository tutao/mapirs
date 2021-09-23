use std::io::Write;
// NOTE: enables creation_flags on the command builder, only works on windows
use std::os::windows::process::CommandExt;
use std::process::Command;

use crate::environment::{client_path, current_time_millis, log_file};
use crate::structs::Message;

const CREATE_NO_WINDOW: u32 = 0x08000000;
const DETACHED_PROCESS: u32 = 0x00000008;

pub fn send_mail(msg: &Message) -> std::io::Result<()> {
    let exe = client_path()?;
    msg.ensure_attachments()?;

    Command::new(&exe)
        .args(&[msg.make_mailto_link()])
        .creation_flags(DETACHED_PROCESS | CREATE_NO_WINDOW)
        .spawn()?;
    log_to_file("send_mail", "spawned tutanota client");
    Ok(())
}

pub fn log_to_file(caller: &str, stuff: &str) {
    let written = if let Ok(mut lf) = log_file() {
        writeln!(lf, "{} | {}: {}", current_time_millis(), caller, stuff)
    } else {
        eprintln!("Couldn't open file");
        Ok(())
    };
    if written.is_err() {
        eprintln!("Couldn't write to file");
    }
}


