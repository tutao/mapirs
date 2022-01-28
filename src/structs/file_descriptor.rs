use std::convert::{From, TryFrom};
#[cfg(not(test))]
use std::fs;
use std::path::PathBuf;

use crate::commands::log_to_file;
use crate::environment::make_subfolder_name_from_content;
use crate::ffi::conversion;
use crate::file_path::FilePath;
use crate::flags::MapiFileFlags;
use crate::types::*;

const FALLBACK_TMP_SUBDIR_PATH: &str = "xxxxxxxx";

#[repr(C)]
#[derive(Debug)]
pub struct RawMapiFileTagExt {
    // ULONG ulReserved - reserved, must be zero
    reserved: ULong,
    // ULONG cbTag - size in bytes of the value defined by the lpTag member.
    cb_tag: ULong,
    // LPBYTE lpTag - X.400 OID for this attachment type
    lp_tag: LpByte,
    // ULONG cbEncoding - size in bytes of
    cb_encoding: ULong,
    // LPBYTE lpEncoding - X.400 OID for this attachment's encoding
    lp_encoding: LpByte,
}

#[derive(Debug)]
pub struct FileTagExtension {
    _tag: Vec<u8>,
    _encoding: Vec<u8>,
}

impl TryFrom<*const RawMapiFileTagExt> for FileTagExtension {
    type Error = ();
    fn try_from(raw_ptr: *const RawMapiFileTagExt) -> Result<Self, Self::Error> {
        if raw_ptr.is_null() {
            Err(())
        } else {
            /*
            SAFETY: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#dereferencing-a-raw-pointer
            Raw Pointers:
            * Are allowed to ignore the borrowing rules by having both immutable and mutable
              pointers or multiple mutable pointers to the same location:
                -> we don't copy these pointers or mutate the pointees, so the only way this can
                   cause problems would be a bug in the calling app
            * Aren’t guaranteed to point to valid memory:
                -> this would be a bug in the calling app, we're using repr(C) to make
                   RawMapiFileTagExt as defined in mapi.h
            * Are allowed to be null:
                -> we checked that
            * Don’t implement any automatic cleanup:
                -> we got the ptr over ffi, so the calling app needs to clean this up
            */
            let raw = unsafe { &*raw_ptr };
            Ok(FileTagExtension {
                _tag: conversion::copy_c_array_to_vec(raw.lp_tag, raw.cb_tag as usize),
                _encoding: conversion::copy_c_array_to_vec(
                    raw.lp_encoding,
                    raw.cb_encoding as usize,
                ),
            })
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct RawMapiFileDesc {
    // ULONG  ulReserved - must be zero
    reserved: ULong,
    // ULONG  flFlags - flags
    flags: MapiFileFlags,
    // ULONG  nPosition - character in text to be replaced by attachment
    position: ULong,
    // LPSTR  lpszPathName - full path name of attachment file
    pub path_name: LpStr,
    // LPSTR  lpszFileName - original file name (optional)
    file_name: LpStr,
    // LPVOID lpFileType - attachment file type (can be lpMapiFileTagExt)
    file_type: *const RawMapiFileTagExt,
}

#[derive(Debug)]
pub struct FileDescriptor {
    _flags: MapiFileFlags,
    _position: ULong,
    /// absolute path to attachment
    pub path_name: FilePath,
    /// file name to use for the attachment (if different from the name in the path)
    pub file_name: Option<PathBuf>,
    _file_type: Option<FileTagExtension>,
}

impl TryFrom<&RawMapiFileDesc> for FileDescriptor {
    type Error = ();

    fn try_from(raw: &RawMapiFileDesc) -> Result<Self, Self::Error> {
        if let Some(file_path) =
            conversion::maybe_string_from_raw_ptr(raw.path_name).map(PathBuf::from)
        {
            let file_path: FilePath = FilePath::try_from(file_path)?;
            Ok(FileDescriptor {
                _flags: raw.flags,
                _position: raw.position,
                path_name: file_path,
                file_name: conversion::maybe_string_from_raw_ptr(raw.file_name).map(PathBuf::from),
                _file_type: FileTagExtension::try_from(raw.file_type).ok(),
            })
        } else {
            Err(())
        }
    }
}

impl FileDescriptor {
    pub fn new(file_path: &str, file_name: Option<&str>) -> Self {
        Self {
            _flags: MapiFileFlags::empty(),
            _position: 0,
            path_name: FilePath::try_from(PathBuf::from(file_path)).unwrap(),
            file_name: file_name.map(PathBuf::from),
            _file_type: None,
        }
    }

    /// check if the last component of the file descriptor's path is different from its file_name.
    /// returns false if file_name is None
    fn needs_new_name(&self) -> bool {
        let file_name = self.file_name.as_ref().map(|pb| pb.file_name()).flatten();

        // this could be a dir name, no easy way to tell
        let path_file_name = self.path_name.file_name();

        match file_name {
            Some(fp) => path_file_name != fp,
            None => false,
        }
    }

    /// take the file at self.path_name and move it to tmp_path + self.file_name if
    /// the self.path_name's last component is not self.file_name and to
    /// tmp_path + basename(self.path_name) otherwise.
    ///
    /// return the path that points to the file to be attached
    pub fn consolidate_into(&self, tmp_path: &Option<PathBuf>) -> PathBuf {
        if tmp_path.is_some() {
            let trg_path_cloned = tmp_path.as_ref().unwrap().clone();
            let trg_name_cloned = if self.needs_new_name() {
                // unwrap is OK because needs_new_name returns false when file_name is None.
                self.file_name.as_ref().unwrap().clone()
            } else {
                self.path_name.file_name().into()
            };

            if let Some(new_path) = self.copy_file_to_tmp_subdir(&trg_path_cloned, &trg_name_cloned)
            {
                return new_path;
            }
        }

        self.path_name.clone().into()
    }

    #[cfg(not(test))]
    fn copy_file_to_tmp_subdir(&self, tmp_path: &PathBuf, tmp_name: &PathBuf) -> Option<PathBuf> {
        let sub_name = make_subfolder_name_from_content(&self.path_name)
            .unwrap_or_else(|_| FALLBACK_TMP_SUBDIR_PATH.to_owned());
        let tmp_subdir_path = tmp_path.join(sub_name);

        if fs::create_dir_all(&tmp_subdir_path).is_err() {
            log_to_file(
                "FileDescriptor::copy_file_to_tmp_subdir",
                "failed to create temporary directory for attachment",
            );
            return None;
        }

        let dest = tmp_subdir_path.join(tmp_name);

        if fs::copy(&self.path_name, &dest).is_err() {
            log_to_file(
                "FileDescriptor::copy_file_to_tmp_subdir",
                "failed to copy file",
            );
            return None;
        }

        Some(dest)
    }

    #[cfg(test)]
    fn copy_file_to_tmp_subdir(&self, tmp_path: &PathBuf, tmp_name: &PathBuf) -> Option<PathBuf> {
        Some(tmp_path.join(FALLBACK_TMP_SUBDIR_PATH).join(tmp_name))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::structs::file_descriptor::FALLBACK_TMP_SUBDIR_PATH;
    use crate::structs::FileDescriptor;

    #[test]
    fn needs_new_name_works() {
        assert!(FileDescriptor::new(&"C:\\hello.txt", Some("ciao.txt")).needs_new_name());

        assert!(!FileDescriptor::new(&"C:\\hello.txt", Some("hello.txt")).needs_new_name());
        assert!(!FileDescriptor::new(&"C:\\hello.txt", None).needs_new_name());
    }

    #[test]
    // TODO test the case where copying the file fails (would require some kind of refactoring, or just a static variable hack)
    fn consolidate_into_works() {
        assert_eq!(
            FileDescriptor::new(&"C:\\User\\Doccies\\hello.txt", Some("hello.txt"))
                .consolidate_into(&Some("C:\\User\\TmpDir".into())),
            PathBuf::from(format!(
                "C:\\User\\TmpDir\\{}\\hello.txt",
                FALLBACK_TMP_SUBDIR_PATH
            )),
            "If the same file name is given, then it is copied with the same filename"
        );

        assert_eq!(
            FileDescriptor::new(&"C:\\User\\Doccies\\hello.txt", Some("ciao.txt"))
                .consolidate_into(&Some("C:\\User\\TmpDir".into())),
            PathBuf::from(format!(
                "C:\\User\\TmpDir\\{}\\ciao.txt",
                FALLBACK_TMP_SUBDIR_PATH
            )),
            "If a different file name is given, then it copies with the new filename",
        );

        assert_eq!(
            FileDescriptor::new(&"C:\\User\\Doccies\\hello.txt", None)
                .consolidate_into(&Some("C:\\User\\TmpDir".into())),
            PathBuf::from(format!(
                "C:\\User\\TmpDir\\{}\\hello.txt",
                FALLBACK_TMP_SUBDIR_PATH
            )),
            "If no file name is given, then it copies with the original filename"
        );
    }
}
