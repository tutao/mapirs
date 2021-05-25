use crate::environment::client_path;

use std::process::Command;
// enables creation_flags on the command builder, only works on windows
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;
const DETACHED_PROCESS: u32 = 0x00000008;

pub fn send_mail() -> () {
	let exe = client_path().unwrap();
	Command::new(&exe)
		// .args(&["/C", "start", &exe])
		.creation_flags(DETACHED_PROCESS)
		.spawn()
		.unwrap();
}

pub fn send_mail_blocking() -> () {
	let mut child = Command::new(&client_path().unwrap())
		.spawn()
		.unwrap();
	child.wait().unwrap();
}
