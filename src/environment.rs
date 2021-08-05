use std::ffi::OsString;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use directories::BaseDirs;
#[cfg(target_os = "windows")]
use winreg::{enums::*, RegKey};

/// access the registry to try and get
/// an OsString containing the absolute path to
/// the tutanota desktop executable.
#[cfg(target_os = "windows")]
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
    let path_string: String = tutanota_key.get_value("InstallLocation")?;
    let mut path_buf = PathBuf::from(path_string);
    path_buf.push("Tutanota Desktop.exe");
    Ok(path_buf.into())
}

#[cfg(not(target_os = "windows"))]
pub fn client_path() -> io::Result<OsString> {
    Ok(OsString::new())
}

/// try to get a file handle to
/// a log file inside the tutanota
/// desktop user data directory.
pub fn log_file() -> io::Result<File> {
    let base_dirs = BaseDirs::new().ok_or(io::Error::new(
        io::ErrorKind::Other,
        "Could not access BaseDirs",
    ))?;
    let data_dir = base_dirs.data_dir();
    let logpath = data_dir.join("tutanota-desktop").join("logs");
    let logfile = logpath.join("mapi.log");

    // this may fail if the path is not writable
    fs::create_dir_all(logpath)?;

    OpenOptions::new()
        .write(true)
        .append(true)
        .open(logfile.clone()) // PathBuf is not copy
        .or_else(|_| File::create(logfile))
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
