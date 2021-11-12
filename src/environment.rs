use std::ffi::OsString;
use std::fmt::Write;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::prelude::DateTime;
use chrono::Utc;
use sha2::{Digest, Sha256};
use winreg::{enums::*, RegKey};

fn reg_key() -> io::Result<RegKey> {
    // it would be possible to get the path via hkcu/software/{tutanota GUID}, but that GUID is
    // different for release, test and snapshot.
    // the GUID is the AppID of Tutanota Desktop as assigned by electron-builder
    // let subkey_path_release = "SOFTWARE\\450699d2-1c81-5ee5-aec6-08dddb7af9d7"

    // the client saves the path to the executable to hklm/software/Clients/Mail/tutanota/EXEPath
    // that key must be there, otherwise windows couldn't have called this DLL because
    // the path to it is next to it under DLLPath.

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    // if this fails, the client is not installed correctly or the registry is borked.
    hklm.open_subkey("SOFTWARE\\Clients\\Mail\\tutanota")
}

/// access the registry to try and get
/// an OsString containing the absolute path to
/// the tutanota desktop executable that registered the dll
/// as the MAPI handler.
pub fn client_path() -> io::Result<OsString> {
    let tutanota_key = reg_key()?;
    // if this fails, the registry is borked.
    tutanota_key.get_value("EXEPath")
}

#[cfg(not(test))]
fn log_path() -> io::Result<OsString> {
    let tutanota_key = reg_key()?;
    tutanota_key.get_value("LOGPath")
}

#[cfg(test)]
fn log_path() -> io::Result<OsString> {
    Ok(OsString::from("C:\\some\\weird\\path"))
}

/// retrieve the configured tmp dir from the registry and
/// try to ensure the directory is there.
#[cfg(not(test))]
pub fn tmp_path() -> io::Result<OsString> {
    let tutanota_key = reg_key()?;
    let tmp_dir = tutanota_key.get_value("TMPPath")?;
    fs::create_dir_all(&tmp_dir)?;
    Ok(tmp_dir)
}

#[cfg(test)]
pub fn tmp_path() -> io::Result<OsString> {
    Ok(OsString::from("C:\\tmp"))
}

/// try to get a file handle to
/// a log file inside the tutanota
/// desktop user data directory.
pub fn log_file() -> io::Result<File> {
    let logpath: PathBuf = log_path()?.into();
    let mut logfile = logpath.clone();
    let mut logfile_old = logpath.clone();
    logfile.push("mapi.log");
    logfile_old.push("mapi_old.log");

    // this may fail if the path is not writable
    fs::create_dir_all(logpath)?;

    // log rotation. if the log was last modified more than a day ago,
    // move it and start a new one.
    if !modified_within_day(&logfile) {
        if let Err(_e) = fs::rename(&logfile, &logfile_old) {
            eprintln!("could not rotate logs.");
        };
    }

    OpenOptions::new()
        .write(true)
        .append(true)
        .open(&logfile)
        .or_else(|_| File::create(&logfile))
}

/// check if the file at a path was modified less than a day ago
/// ignores pretty much any error, returning false
fn modified_within_day<P: AsRef<Path>>(filepath: P) -> bool {
    if let Some(v) = fs::metadata(filepath)
        .ok()
        .map(|md| md.modified().ok())
        .flatten()
        .map(|modified| SystemTime::now().duration_since(modified).ok())
        .flatten()
        .map(|dur| dur.as_secs() < 60 * 60 * 24)
    {
        v
    } else {
        false
    }
}

/// we may get the same filename multiple times
/// we put each file into its own subfolder that's named
/// after the first 4 characters of the hex-encoded SHA256 hash
/// of the file contents
pub fn make_subfolder_name_from_content<P: AsRef<Path>>(filepath: P) -> io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut sha256 = Sha256::new();
    io::copy(&mut file, &mut sha256)?;
    Ok(sha_head(sha256.finalize()))
}

pub fn sha_head(
    sha256: sha2::digest::generic_array::GenericArray<u8, <Sha256 as Digest>::OutputSize>,
) -> String {
    let mut buf = String::with_capacity(4);
    for byte in &sha256[..2] {
        if write!(buf, "{:>2x}", byte).is_err() {
            return "nope".to_owned();
        };
    }
    buf
}

/// get the current system time as a formatted string
pub fn current_time_formatted() -> String {
    let date_time = DateTime::<Utc>::from(SystemTime::now());
    date_time.format("%Y-%m-%d | %H:%M:%S.%3f").to_string()
}

#[cfg(test)]
mod test {
    #[test]
    fn sha_head_works() {
        use crate::environment::sha_head;
        use sha2::{Digest, Sha256};

        let sha256 = Sha256::new();
        let out = sha_head(sha256.finalize());
        assert_eq!("e3b0", out);
        assert_eq!(4, out.capacity());
    }
}
