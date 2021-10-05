use std::convert::{From, TryFrom};
#[cfg(not(test))]
use std::fs;
use std::path::PathBuf;

use crate::ffi::conversion;
use crate::flags::MapiFileFlags;
use crate::types::*;

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
    tag: Vec<u8>,
    encoding: Vec<u8>,
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
                tag: conversion::copy_c_array_to_vec(raw.lp_tag, raw.cb_tag as usize),
                encoding: conversion::copy_c_array_to_vec(
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
    flags: MapiFileFlags,
    position: ULong,
    /// absolute path to attachment
    pub path_name: PathBuf,
    /// file name to use for the attachment (if different from the name in the path)
    pub file_name: Option<PathBuf>,
    file_type: Option<FileTagExtension>,
}

impl TryFrom<&RawMapiFileDesc> for FileDescriptor {
    type Error = ();

    fn try_from(raw: &RawMapiFileDesc) -> Result<Self, Self::Error> {
        if let Some(file_path) =
            conversion::maybe_string_from_raw_ptr(raw.path_name).map(PathBuf::from)
        {
            Ok(FileDescriptor {
                flags: raw.flags,
                position: raw.position,
                path_name: file_path,
                file_name: conversion::maybe_string_from_raw_ptr(raw.file_name).map(PathBuf::from),
                file_type: FileTagExtension::try_from(raw.file_type).ok(),
            })
        } else {
            Err(())
        }
    }
}

impl FileDescriptor {
    #[cfg(test)]
    pub fn new(file_path: &str, file_name: Option<&str>) -> Self {
        Self {
            flags: MapiFileFlags::empty(),
            position: 0,
            path_name: PathBuf::from(file_path),
            file_name: file_name.map(PathBuf::from),
            file_type: None,
        }
    }

    /// check if the last component of the file descriptor's path is the same as its file name.
    /// returns false if file_name does not contain a file name or file_path is the root dir
    fn needs_consolidation(&self) -> bool {
        let file_name = self.file_name.as_ref().map(|pb| pb.file_name()).flatten();

        // this could be a dir name, no easy way to tell
        // will be none if the last element is '..' or if
        // file_path is the root
        let path_file_name = self.path_name.file_name();

        if path_file_name.is_none() || file_name.is_none() {
            return false;
        }

        path_file_name != file_name
    }

    /// take the file at self.path_name and move it to tmp_path + self.file_name if
    /// the first's last component is not self.file_name and the latter makes sense.
    ///
    /// return the path that points to the file to be attached
    #[cfg(not(test))]
    pub fn consolidate_into(&self, tmp_path: &Option<PathBuf>) -> PathBuf {
        if tmp_path.is_some() && self.needs_consolidation() {
            // unwrap is OK because needs_consolidation returns false when file_name is None.
            let trg_cloned = tmp_path.as_ref().unwrap().clone();
            let trg_name_cloned = self.file_name.as_ref().unwrap().clone();
            let new_path = trg_cloned.join(trg_name_cloned);
            if fs::copy(&self.path_name, &new_path).is_ok() {
                return new_path;
            }
        }

        self.path_name.clone()
    }

    #[cfg(test)]
    pub fn consolidate_into(&self, tmp_path: &Option<PathBuf>) -> PathBuf {
        if tmp_path.is_some() && self.needs_consolidation() {
            // unwrap is OK because needs_consolidation returns false when file_name is None.
            let trg_cloned = tmp_path.as_ref().unwrap().clone();
            let trg_name_cloned = self.file_name.as_ref().unwrap().clone();
            let new_path = trg_cloned.join(trg_name_cloned);
            return new_path;
        }

        self.path_name.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::FileDescriptor;

    #[test]
    fn needs_consolidation_works() {
        assert!(!FileDescriptor::new(&"C:\\hello.txt", Some("hello.txt")).needs_consolidation());
        assert!(FileDescriptor::new(&"C:\\hello.txt", Some("ciao.txt")).needs_consolidation());
        assert!(!FileDescriptor::new(&"C:\\hello.txt", None).needs_consolidation());
        assert!(!FileDescriptor::new(&"C:\\", Some("hello.txt")).needs_consolidation());
        assert!(!FileDescriptor::new(&"C:\\", None).needs_consolidation());
    }
}
