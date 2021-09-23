use std::convert::TryFrom;
use std::fs;
use std::io;

use urlencoding::encode;

use crate::environment;
use crate::ffi::conversion;
use crate::flags::MapiMessageFlags;
use crate::structs::{FileDescriptor, RawMapiFileDesc, RawMapiRecipDesc, RecipientDescriptor};
use crate::types::*;

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/ns-mapi-mapimessage
#[repr(C)]
#[derive(Debug)]
pub struct RawMapiMessage {
    reserved: ULong,
    // ULONG ulReserved - reserved, must be 0 or CP_UTF8
    subject: LpStr,
    // LPSTR lpszSubject - message subject
    note_text: LpStr,
    // LPSTR lpszNoteText - message text
    message_type: LpStr,
    // LPSTR lpszMessageType - message class
    date_received: LpStr,
    // LPSTR lpszDateReceived - in YYYY/MM/DD HH:MM format
    conversation_id: LpStr,
    // LPSTR lpszConversationID - conversation thread id
    flags: MapiMessageFlags,
    // TODO: FLAGS flFlags - unread, return receipt
    originator: *const RawMapiRecipDesc,
    // TODO: lpMapiRecipDesc lpOriginator - originator descriptor
    recip_count: ULong,
    // ULONG nRecipCount - number of recipients
    recips: *const RawMapiRecipDesc,
    // TODO: lpMapiRecipDesc lpRecips - recipient descriptors
    file_count: ULong,
    // ULONG nFileCount - # of file attachments
    files: *const RawMapiFileDesc,       // TODO: lpMapiFileDesc lpFiles - attachment descriptors
}

#[derive(Debug)]
pub struct Message {
    subject: Option<String>,
    note_text: Option<String>,
    message_type: Option<String>,
    date_received: Option<String>,
    conversation_id: Option<String>,
    flags: MapiMessageFlags,
    originator: Option<RecipientDescriptor>,
    recips: Vec<RecipientDescriptor>,
    files: Vec<FileDescriptor>,
}

impl TryFrom<*const RawMapiMessage> for Message {
    type Error = ();
    fn try_from(raw_ptr: *const RawMapiMessage) -> Result<Self, Self::Error> {
        if raw_ptr.is_null() {
            Err(())
        } else {
            let raw = unsafe { &*raw_ptr };
            let originator_result = RecipientDescriptor::try_from(raw.originator);
            let recips: Vec<RecipientDescriptor> = conversion::raw_to_vec(raw.recips, raw.recip_count as usize);
            let files: Vec<FileDescriptor> = conversion::raw_to_vec(raw.files, raw.file_count as usize);
            Ok(Message {
                subject: conversion::maybe_string_from_raw_ptr(raw.subject),
                note_text: conversion::maybe_string_from_raw_ptr(raw.note_text),
                message_type: conversion::maybe_string_from_raw_ptr(raw.message_type),
                date_received: conversion::maybe_string_from_raw_ptr(raw.date_received),
                conversation_id: conversion::maybe_string_from_raw_ptr(raw.conversation_id),
                flags: raw.flags,
                originator: originator_result.ok(),
                recips,
                files,
            })
        }
    }
}

impl Message {
    /// FileDescriptors may have a path and a separate file name
    /// to make it easier, copy the attachment to the file name
    /// next to the path.
    /// don't do anything if file name is the same as the path pointed to
    /// or if the attachment file descriptor doesn't have a separate file
    /// name.
    pub fn ensure_attachments(&self) -> io::Result<()> {
        for file_desc in &self.files {
            let maybe_path = environment::swap_filename(&file_desc.path_name, &file_desc.file_name);
            let new_path = if let Some(np) = maybe_path {
                np
            } else {
                continue;
            };
            fs::copy(&file_desc.path_name, &new_path)?;
        }
        Ok(())
    }

    pub fn make_mailto_link(&self) -> String {
        // MAPI message only has a recipient array, so we use the first one for the
        // address and put the rest (comma-separated) into cc.
        let to = self
            .recips
            .get(0)
            .map(|r| r.address.clone())
            .unwrap_or_else(|| Some("".to_owned()))
            .unwrap();
        let cc = self
            .recips
            .iter()
            .skip(1)
            .filter_map(|r| r.address.clone())
            .collect::<Vec<String>>();
        let subject = self
            .subject
            .as_ref().cloned();
        let body = self
            .note_text
            .as_ref().cloned();

        // takes the file descriptors and make file urls from them
        let fd_mapper = |fd: &FileDescriptor| environment::swap_filename(
            &fd.path_name,
            &fd.file_name,
        ).unwrap_or_else(|| fd.path_name.clone());

        let attachments = self
            .files
            .iter()
            .map(fd_mapper);

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

        for attachment in attachments {
            if let Some(fp) = attachment.to_str() {
                url_parts.push(format!("attach={}", encode(fp)));
            }
        }

        format!("mailto:{}?{}", to, url_parts.join("&"))
    }

    #[cfg(test)]
    pub fn new(to: Vec<&str>, body: Option<&str>, subject: Option<&str>, attach: Vec<FileDescriptor>) -> Self {
        Self {
            subject: subject.map(|s| s.to_owned()),
            note_text: body.map(|b| b.to_owned()),
            message_type: None,
            date_received: None,
            conversation_id: None,
            flags: MapiMessageFlags::empty(),
            originator: None,
            recips: to.into_iter().map(|t| RecipientDescriptor::new(t)).collect(),
            files: attach,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::{FileDescriptor, Message};

    #[test]
    fn message_make_mailto_works() {
        assert_eq!(Message::new(
            vec![],
            None,
            None,
            vec![],
        ).make_mailto_link(), "mailto:?");

        assert_eq!(Message::new(
            vec!["a@b.de", "b@c.de", "d@g.de"],
            None,
            None,
            vec![],
        ).make_mailto_link(), "mailto:a@b.de?cc=b@c.de,d@g.de");

        assert_eq!(Message::new(
            vec!["a@b.de"],
            None,
            None,
            vec![FileDescriptor::new("C:\\some\\path file.jpg", "file.txt".into())],
        ).make_mailto_link(), "mailto:a@b.de?attach=C%3A%5Csome%5Cfile.txt");

        assert_eq!(Message::new(
            vec!["a@b.de"],
            None,
            None,
            vec![FileDescriptor::new("C:\\some\\path file.jpg", None)],
        ).make_mailto_link(), "mailto:a@b.de?attach=C%3A%5Csome%5Cpath%20file.jpg");

        assert_eq!(Message::new(
            vec!["a@b.de"],
            "börk & ? = / \\".into(),
            "börk & ? \\ %20 ".into(),
            vec![],
        ).make_mailto_link(), "mailto:a@b.de?subject=b%C3%B6rk%20%26%20%3F%20%5C%20%2520%20&body=b%C3%B6rk%20%26%20%3F%20%3D%20%2F%20%5C");
    }
}