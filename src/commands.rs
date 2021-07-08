use crate::environment::client_path;
use crate::structs::Message;

use std::process::Command;
// NOTE: enables creation_flags on the command builder, only works on windows
#[cfg(windows)]
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;
const DETACHED_PROCESS: u32 = 0x00000008;

#[cfg(target_os = "windows")]
pub fn send_mail(msg: &Message) -> () {
    let exe = client_path().unwrap();
    Command::new(&exe)
        .args(&[msg.make_mailto_link()])
        .creation_flags(DETACHED_PROCESS)
        .spawn()
        .unwrap();
}

#[cfg(not(target_os = "windows"))]
pub fn send_mail(_msg: &Message) -> () {}
