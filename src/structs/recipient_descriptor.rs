use std::convert::TryFrom;

use crate::ffi::conversion;
use crate::types::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawMapiRecipDesc {
    // ULONG ulReserved - reserved for future use
    reserved: ULong,
    // ULONG ulRecipClass - recipient class
    recip_class: ULong,
    // LPSTR lpszName - recipient name
    name: LpStr,
    // LPSTR lpszAddress - recitpient address (optional)
    address: LpStr,
    // ULONG ulEIDSize count in bytes of size of pEntryID
    eid_size: ULong,
    // LPVOID lpEntryID system-specific recipient reference
    entry_id: *const libc::c_uchar,
}

#[derive(Debug)]
pub struct RecipientDescriptor {
    _recip_class: ULong,
    _name: String,
    pub address: Option<String>,
    _entry_id: Vec<u8>,
}

impl TryFrom<*const RawMapiRecipDesc> for RecipientDescriptor {
    type Error = ();
    fn try_from(raw_ptr: *const RawMapiRecipDesc) -> Result<Self, Self::Error> {
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
                   RawMapiRecipDesc as defined in mapi.h
            * Are allowed to be null:
                -> we checked that
            * Don’t implement any automatic cleanup:
                -> we got the ptr over ffi, so the calling app needs to clean this up
            */
            let raw: &RawMapiRecipDesc = unsafe { &*raw_ptr };
            Ok(Self::from(raw))
        }
    }
}

impl From<&RawMapiRecipDesc> for RecipientDescriptor {
    fn from(raw: &RawMapiRecipDesc) -> Self {
        // some applications (Sage50) prefix the mail addresses with SMTP: which is
        // technically not valid, but we're going to make a best effort to allow this.
        // ":" is only allowed in quoted local parts so we're not going to destroy
        // valid mail addresses with this.
        let address = conversion::maybe_string_from_raw_ptr(raw.address).map(|a| {
            if a.starts_with("SMTP:") {
                a.strip_prefix("SMTP:").unwrap().to_owned()
            } else {
                a
            }
        });

        RecipientDescriptor {
            _recip_class: raw.recip_class,
            _name: conversion::maybe_string_from_raw_ptr(raw.name)
                .unwrap_or_else(|| "MISSING_RECIP_NAME".to_owned()),
            address,
            _entry_id: conversion::copy_c_array_to_vec(raw.entry_id, raw.eid_size as usize),
        }
    }
}

impl RecipientDescriptor {
    #[cfg(test)]
    pub fn new(address: &str) -> Self {
        Self {
            _recip_class: 0,
            _name: "".to_owned(),
            address: Some(address.to_owned()),
            _entry_id: vec![0, 0, 0, 0],
        }
    }
}

#[cfg(test)]
mod test {
    use std::ffi::CStr;

    use crate::structs::{RawMapiRecipDesc, RecipientDescriptor};

    #[test]
    fn smtp_prefix_is_stripped() {
        let raw = |a: &str| RawMapiRecipDesc {
            reserved: 0,
            recip_class: 0,
            name: std::ptr::null(),
            address: CStr::from_bytes_with_nul(a.as_bytes()).unwrap().as_ptr(),
            eid_size: 0,
            entry_id: std::ptr::null(),
        };

        let address1 = raw(&"SMTP:a@b.c\0");
        let address2 = raw(&"\"SMTP:a\"@b.c\0");
        assert_eq!(
            RecipientDescriptor::from(&address1).address,
            Some("a@b.c".to_owned())
        );
        assert_eq!(
            RecipientDescriptor::from(&address2).address,
            Some("\"SMTP:a\"@b.c".to_owned())
        );
    }
}
