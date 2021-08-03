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
        if std::ptr::null() == raw_ptr {
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
        let to = self
            .recips
            .iter()
            .nth(1)
            .map(|r| r.address.clone())
            .unwrap_or(Some("".to_owned()));
        let cc = self
            .recips
            .iter()
            .skip(1)
            .filter_map(|r| r.address.clone())
            .collect::<Vec<String>>()
            .join(",");
        let subject = self
            .subject
            .as_ref()
            .map(|s| s.clone())
            .unwrap_or("".to_owned());
        let body = self
            .note_text
            .as_ref()
            .map(|s| s.clone())
            .unwrap_or("".to_owned());
        let attachments = self
            .files
            .iter()
            .map(|fd| environment::swap_filename(&fd.path_name, &fd.file_name).unwrap_or(fd.path_name.clone()));

        format!(
            "mailto:{}?cc={}&subject={}&body={}",
            to.unwrap(),
            cc,
            encode(&subject),
            encode(&body)
        )
    }
}
