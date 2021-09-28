use std::convert::{From, TryFrom};
use std::path::PathBuf;

use crate::ffi::conversion;
use crate::flags::MapiFileFlags;
use crate::types::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawMapiFileTagExt {
    reserved: ULong,
    // ULONG ulReserved - reserved, must be zero
    cb_tag: ULong,
    // ULONG cbTag - size in bytes of the value defined by the lpTag member.
    lp_tag: LpByte,
    // LPBYTE lpTag - X.400 OID for this attachment type
    cb_encoding: ULong,
    // ULONG cbEncoding - size in bytes of
    lp_encoding: LpByte, // LPBYTE lpEncoding - X.400 OID for this attachment's encoding
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
                encoding: conversion::copy_c_array_to_vec(raw.lp_encoding, raw.cb_encoding as usize),
            })
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct RawMapiFileDesc {
    reserved: ULong,
    // ULONG  ulReserved - must be zero
    flags: MapiFileFlags,
    // ULONG  flFlags - flags
    position: ULong,
    // ULONG  nPosition - character in text to be replaced by attachment
    pub path_name: LpStr,
    // LPSTR  lpszPathName - full path name of attachment file
    file_name: LpStr,
    // LPSTR  lpszFileName - original file name (optional)
    file_type: *const RawMapiFileTagExt, // LPVOID lpFileType - attachment file type (can be lpMapiFileTagExt)
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
        if let Some(file_path) = conversion::maybe_string_from_raw_ptr(raw.path_name)
            .map(PathBuf::from) {
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
}