use std::convert::TryFrom;
use std::path::PathBuf;

use urlencoding::encode;

use crate::commands::log_to_file;
use crate::environment;
use crate::ffi::conversion;
use crate::flags::MapiMessageFlags;
use crate::structs::{FileDescriptor, RawMapiFileDesc, RawMapiRecipDesc, RecipientDescriptor};
use crate::types::*;

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/ns-mapi-mapimessage
#[repr(C)]
#[derive(Debug)]
pub struct RawMapiMessage {
    // ULONG ulReserved - reserved, must be 0 or CP_UTF8
    reserved: ULong,
    // LPSTR lpszSubject - message subject
    subject: LpStr,
    // LPSTR lpszNoteText - message text
    note_text: LpStr,
    // LPSTR lpszMessageType - message class
    message_type: LpStr,
    // LPSTR lpszDateReceived - in YYYY/MM/DD HH:MM format
    date_received: LpStr,
    // LPSTR lpszConversationID - conversation thread id
    conversation_id: LpStr,
    // FLAGS flFlags
    flags: MapiMessageFlags,
    // lpMapiRecipDesc lpOriginator
    originator: *const RawMapiRecipDesc,
    // ULONG nRecipCount - number of recipients
    recip_count: ULong,
    // lpMapiRecipDesc lpRecips - recipient descriptors
    recips: *const RawMapiRecipDesc,
    // ULONG nFileCount - # of file attachments
    file_count: ULong,
    // lpMapiFileDesc lpFiles - attachment descriptors
    files: *const RawMapiFileDesc,
}

#[derive(Debug)]
pub struct Message {
    subject: Option<String>,
    note_text: Option<String>,
    _message_type: Option<String>,
    _date_received: Option<String>,
    _conversation_id: Option<String>,
    _flags: MapiMessageFlags,
    _originator: Option<RecipientDescriptor>,
    recips: Vec<RecipientDescriptor>,
    files: Vec<FileDescriptor>,
}

impl TryFrom<*const RawMapiMessage> for Message {
    type Error = ();
    fn try_from(raw_ptr: *const RawMapiMessage) -> Result<Self, Self::Error> {
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
                   RawMapiMessage as defined in mapi.h
            * Are allowed to be null:
                -> we checked that
            * Don’t implement any automatic cleanup:
                -> we got the ptr over ffi, so the calling app needs to clean this up
            */
            let raw = unsafe { &*raw_ptr };
            let originator_result = RecipientDescriptor::try_from(raw.originator);
            let recips: Vec<RecipientDescriptor> =
                conversion::raw_to_vec(raw.recips, raw.recip_count as usize)
                    .into_iter()
                    .flatten()
                    .collect();
            if recips.len() < raw.recip_count as usize {
                log_to_file(
                    "Message::from::<RawMapiMessage>",
                    "could not parse one or more RecipientDescriptors",
                );
            }
            let files: Vec<FileDescriptor> = conversion::raw_to_vec::<
                FileDescriptor,
                RawMapiFileDesc,
            >(raw.files, raw.file_count as usize)
            .into_iter()
            .flatten()
            .collect();
            if files.len() < raw.file_count as usize {
                log_to_file(
                    "Message::from::<RawMapiMessage>",
                    "could not parse one or more FileDescriptors",
                );
            }
            Ok(Message {
                subject: conversion::maybe_string_from_raw_ptr(raw.subject),
                note_text: conversion::maybe_string_from_raw_ptr(raw.note_text),
                _message_type: conversion::maybe_string_from_raw_ptr(raw.message_type),
                _date_received: conversion::maybe_string_from_raw_ptr(raw.date_received),
                _conversation_id: conversion::maybe_string_from_raw_ptr(raw.conversation_id),
                _flags: raw.flags,
                _originator: originator_result.ok(),
                recips,
                files,
            })
        }
    }
}

impl Message {
    /// Copy the files to be attached to a temp directory that's accessible by tutanota.
    /// it copies the file from the file path to the temp directory and renames it so the
    /// name matches file_name (if present).
    ///
    /// Reasons for doing this:
    /// * FileDescriptors may have a path and a separate file name, but we only have a single path
    /// to pass in the mailto-url, so the file name at the end of the path should be what's in the
    /// file descriptor.
    ///
    /// * we return as soon as we send the command to start tutanota, and some applications using
    /// mapi to send attachments will delete the file they passed as soon as they get back control,
    /// like Adobe Acrobat Reader.
    ///
    /// This will lead to some files being attached from an unexpected location, but it is
    /// preferable to copying the file next to the one with the wrong name and possibly clobbering
    /// other files or ignoring file_name.
    pub fn ensure_attachments(&self) -> Vec<PathBuf> {
        let tmp_path: Option<PathBuf> = environment::tmp_path().ok().map(|p| p.into());
        self.files
            .iter()
            .map(|desc| desc.consolidate_into(&tmp_path))
            .collect()
    }

    pub fn make_mailto_link(&self) -> String {
        // MAPI message only has a recipient array, so we use the first one for the
        // address and put the rest (comma-separated) into cc.
        let to = self
            .recips
            .first()
            .map(|r| r.address.clone())
            .unwrap_or_else(|| Some("".to_owned()))
            .unwrap();
        let cc = self
            .recips
            .iter()
            .skip(1)
            .filter_map(|r| r.address.clone())
            .collect::<Vec<String>>();
        let subject = self.subject.as_ref().cloned();
        let body = self.note_text.as_ref().cloned();

        let mut url_parts = vec![];

        if !cc.is_empty() {
            url_parts.push(format!("cc={}", cc.join(",")));
        }

        if let Some(subject_text) = subject {
            url_parts.push(format!("subject={}", encode(&subject_text)));
        }

        if let Some(body_text) = body {
            url_parts.push(format!("body={}", encode(&body_text)));
        }

        for attachment in self.ensure_attachments() {
            if let Some(fp) = attachment.to_str() {
                url_parts.push(format!("attach={}", encode(fp)));
            }
        }
        let lnk = format!("mailto:{}?{}", to, url_parts.join("&"));
        log_to_file("make_mailto", "finished");
        lnk
    }

    #[cfg(test)]
    pub fn new(
        to: Vec<&str>,
        body: Option<&str>,
        subject: Option<&str>,
        attach: Vec<FileDescriptor>,
    ) -> Self {
        Self {
            subject: subject.map(|s| s.to_owned()),
            note_text: body.map(|b| b.to_owned()),
            _message_type: None,
            _date_received: None,
            _conversation_id: None,
            _flags: MapiMessageFlags::empty(),
            _originator: None,
            recips: to
                .into_iter()
                .map(|t| RecipientDescriptor::new(t))
                .collect(),
            files: attach,
        }
    }

    pub fn from_paths(paths: Vec<String>, names: Vec<String>) -> Self {
        // if we got file names, but not the the same amount as paths, we
        // will use the file paths as-is
        let names: Vec<Option<&str>> = if names.is_empty() || names.len() != paths.len() {
            vec![None; paths.len()]
        } else {
            names.iter().map(AsRef::as_ref).map(Some).collect()
        };

        let files = paths
            .into_iter()
            .zip(names.into_iter())
            .map(|(p, n)| FileDescriptor::new(&p, n))
            .collect();

        Self {
            subject: None,
            note_text: None,
            _message_type: None,
            _date_received: None,
            _conversation_id: None,
            _flags: MapiMessageFlags::empty(),
            _originator: None,
            recips: vec![],
            files,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::{FileDescriptor, Message};

    #[test]
    fn message_make_mailto_works() {
        assert_eq!(
            Message::new(vec![], None, None, vec![]).make_mailto_link(),
            "mailto:?"
        );

        assert_eq!(
            Message::new(vec!["a@b.de", "b@c.de", "d@g.de"], None, None, vec![]).make_mailto_link(),
            "mailto:a@b.de?cc=b@c.de,d@g.de"
        );

        assert_eq!(
            Message::new(
                vec!["a@b.de"],
                None,
                None,
                vec![FileDescriptor::new(
                    "C:\\some\\path file.jpg",
                    "file.txt".into(),
                )],
            )
            .make_mailto_link(),
            "mailto:a@b.de?attach=C%3A%5Ctmp%5Cxxxxxxxx%5Cfile.txt"
        );

        assert_eq!(
            Message::new(
                vec!["a@b.de"],
                None,
                None,
                vec![FileDescriptor::new("C:\\some\\path file.jpg", None)],
            )
            .make_mailto_link(),
            "mailto:a@b.de?attach=C%3A%5Ctmp%5Cxxxxxxxx%5Cpath%20file.jpg"
        );

        assert_eq!(Message::new(
            vec!["a@b.de"],
            "börk & ? = / \\".into(),
            "börk & ? \\ %20 ".into(),
            vec![],
        ).make_mailto_link(), "mailto:a@b.de?subject=b%C3%B6rk%20%26%20%3F%20%5C%20%2520%20&body=b%C3%B6rk%20%26%20%3F%20%3D%20%2F%20%5C");
    }
}
