use std::ffi::OsString;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::{
    Path, PathBuf,
};
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(target_os = "windows")]
use winreg::{enums::*, RegKey};

#[cfg(target_os = "windows")]
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
#[cfg(target_os = "windows")]
pub fn client_path() -> io::Result<OsString> {
    let tutanota_key = reg_key()?;
    // if this fails, the registry is borked.
    tutanota_key.get_value("EXEPath")
}

#[cfg(not(target_os = "windows"))]
pub fn client_path() -> io::Result<OsString> {
    Ok(OsString::new())
}

#[cfg(target_os = "windows")]
#[cfg(not(test))]
fn log_path() -> io::Result<OsString> {
    let tutanota_key = reg_key()?;
    tutanota_key.get_value("LOGPath")
}

#[cfg(test)]
fn log_path() -> io::Result<OsString> {
    Ok(OsString::from("/some/weird/path"))
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
        if let Err(e) = fs::rename(&logfile, &logfile_old) {
            eprintln!("could not rotate logs.");
        };
    }

    OpenOptions::new()
        .write(true)
        .append(true)
        .open(&logfile) // PathBuf is not copy
        .or_else(|_| File::create(&logfile))
}

/// check if the file at a path was modified less than a day ago
/// ignores pretty much any error, returning false
fn modified_within_day<P: AsRef<Path>>(filepath: P) -> bool {
    if let Some(v) = fs::metadata(filepath).ok()
        .map(|md| md.modified().ok()).flatten()
        .map(|modified| SystemTime::now().duration_since(modified).ok()).flatten()
        .map(|dur| dur.as_secs() < 60 * 60 * 24) {
        v
    } else {
        false
    }
}

/// get the current system time in
/// milliseconds since unix epoch
pub fn current_time_millis() -> u128 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards");
    duration.as_millis()
}

/// get a path to a file in the same directory as file_path but named file_name
///
/// TODO: get this example to run as a doctest on linux?
/// TODO: for now, it's part of the test mod below
/// ```
/// let fpath = PathBuf::from("/home/u/text.txt");
/// let fname = Some(PathBuf::from("image.jpg"));
/// let res1 = swap_filename(&fpath, &fname);
/// assert_eq!(res1.unwrap(), PathBuf::from("/home/u/image.jpg"));
/// ```
///
/// returns None if file_name does not contain a file name or file_path is the root dir
pub fn swap_filename(file_path: &PathBuf, file_name: &Option<PathBuf>) -> Option<PathBuf> {
    // check if the file name is present and get its last component
    let file_name = if let Some(nm) = file_name.as_ref().map(|pb| pb.file_name()).flatten() {
        nm
    } else {
        return None;
    };

    // get the last path component (could be a dir, no way to tell)
    let path_file_name = if let Some(nm) = file_path.file_name() {
        nm
    } else {
        return None;
    };

    // check that the path is not the root
    let dir_path = if let Some(dp) = file_path.parent() {
        dp
    } else {
        return None;
    };

    if path_file_name == file_name {
        return Some(file_path.clone());
    }

    Some(dir_path.join(file_name))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::environment::swap_filename;

    #[test]
    fn swap_filename_works() {
        let fpath1 = PathBuf::from("/home/u/text.txt");
        let fpath2 = PathBuf::from("/");
        let fpath3 = PathBuf::from("/home/no/");
        let fname1 = Some(PathBuf::from("image.jpg"));
        let fname2 = None;
        let fname3 = Some(PathBuf::from("hello/image.jpg"));
        let fname4 = Some(PathBuf::from("text.txt"));
        // normal operation
        let res1 = swap_filename(&fpath1, &fname1);
        assert_eq!(res1.unwrap(), PathBuf::from("/home/u/image.jpg"));
        // if there's no file name, we don't return anything
        let res2 = swap_filename(&fpath1, &fname2);
        assert!(res2.is_none());
        // root dir doesn't have a parent
        let res3 = swap_filename(&fpath2, &fname1);
        assert!(res3.is_none());
        // if file path is a dir, we still want to do our thing.
        let res4 = swap_filename(&fpath3, &fname1);
        assert_eq!(res4.unwrap(), PathBuf::from("/home/image.jpg"));
        // get only the last component (file name) of the second arg
        let res5 = swap_filename(&fpath1, &fname3);
        assert_eq!(res5.unwrap(), PathBuf::from("/home/u/image.jpg"));
        // do nothing if the result is equal to file_path
        let res6 = swap_filename(&fpath1, &fname4);
        assert_eq!(res6.unwrap(), fpath1);
    }
}
