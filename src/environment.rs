use winreg::{
	enums::*,
	RegKey,
};
use directories::BaseDirs;
use std::fs;
use std::fs::{
	OpenOptions,
	File,
};

use std::io;
use std::path::PathBuf;
use std::ffi::OsString;
use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::From;

/// access the registry to try and get
/// an OsString containing the absolute path to 
/// the tutanota desktop executable.
pub fn client_path() -> io::Result<OsString> {
	let subkey_path = "SOFTWARE\\450699d2-1c81-5ee5-aec6-08dddb7af9d7";
	let hkcu = RegKey::predef(HKEY_CURRENT_USER);
	// first, try to find executable for client installed for current user
	let tutanota_key = if let Ok(hkcu_subkey) = hkcu.open_subkey(subkey_path) {
		hkcu_subkey
	} else {
		// if that didn't work, try to get the globally installed executable.
		let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
		// if this fails, the client is not installed or the registry is borked.
		let hklm_subkey = hklm.open_subkey(subkey_path)?;
		hklm_subkey
	};
	// if this fails, the registry is borked.
	let path_string : String = tutanota_key.get_value("InstallLocation")?;
	let mut path_buf = PathBuf::from(path_string);
	path_buf.push("Tutanota Desktop.exe");
	Ok(path_buf.into())
}

/// try to get a file handle to 
/// a log file inside the tutanota 
/// desktop user data directory.
pub fn log_file() -> io::Result<File> {
	let base_dirs = BaseDirs::new()
   		.ok_or(io::Error::new(
   			io::ErrorKind::Other, 
   			"Could not access BaseDirs"
   		))?; 
	let data_dir = base_dirs.data_dir();
	let logpath = data_dir.join("tutanota-desktop");
	let logfile = logpath.join("mapilog.txt");

   	// this may fail if the path is not writable
	fs::create_dir_all(logpath)?;

	OpenOptions::new()
		.write(true)
		.append(true)
		.open(logfile.clone()) // PathBuf is not copy  
		.or_else(|_|{File::create(logfile)})
}

/// get the current system time in
/// milliseconds since unix epoch
pub fn current_time_millis() -> u128 {
	let duration = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.expect("time went backwards");
	duration.as_millis()
}
