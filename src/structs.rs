use std::convert::TryFrom;
use std::convert::From;
use std::convert::Into;
use std::ffi::CStr;
use std::fmt::Debug;
use urlencoding::encode;

use crate::types::*;

use crate::flags::{
	MapiFileFlags,
	MapiMessageFlags,
};

fn maybe_string_from_raw_ptr(ptr: LpStr) -> Option<String> {
	if std::ptr::null() == ptr {
		return None;
	}
	let maybe_str = unsafe {
 		let cstr = CStr::from_ptr(ptr);
 		cstr.to_str().clone()
	};
	
	if let Ok(s) = maybe_str {
		Some(String::from(s))
	} else {
		None
	}
}

fn raw_to_vec<'a, K: From<&'a T>, T: 'a>(ptr: *const T, count: usize) -> Vec<K> {
	let mut v : Vec<K> = vec![];
	if std::ptr::null() == ptr || count == 0 {
		return v;
	}
	let slc : &[T] = unsafe { std::slice::from_raw_parts(ptr, count) };
	for t in slc.iter() {
		v.push(t.into());
	}
	v
}

fn copy_c_array_to_vec<T: Clone>(ptr: *const T, count: usize) -> Vec<T> {
	if std::ptr::null() == ptr || count == 0 {
		vec![]
	} else {
		let slc : &[T] = unsafe { std::slice::from_raw_parts(ptr, count) };
		slc.to_vec()
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct RawMapiFileTagExt {
	reserved: ULong, 			// ULONG ulReserved - reserved, must be zero
	cb_tag: ULong,			// ULONG cbTag - size in bytes of the value defined by the lpTag member.
	lp_tag: LpByte, 	 	// LPBYTE lpTag - X.400 OID for this attachment type
	cb_encoding: ULong,		// ULONG cbEncoding - size in bytes of
	lp_encoding: LpByte,	// LPBYTE lpEncoding - X.400 OID for this attachment's encoding
}

#[derive(Debug)]
pub struct FileTagExtension {
	tag: Vec<u8>,
	encoding: Vec<u8>,
}

impl TryFrom<*const RawMapiFileTagExt> for FileTagExtension {
	type Error = ();
	fn try_from(raw_ptr: *const RawMapiFileTagExt) -> Result<Self, Self::Error> {
		if std::ptr::null() == raw_ptr {
			Err(())
		} else {
			let raw = unsafe {&*raw_ptr};
			Ok(FileTagExtension {
				tag: copy_c_array_to_vec(raw.lp_tag, raw.cb_tag as usize),
				encoding: copy_c_array_to_vec(raw.lp_encoding, raw.cb_encoding as usize),
			})
		}
	}
}


#[repr(C)]
#[derive(Debug)]
pub struct RawMapiFileDesc {
  reserved: ULong, // ULONG  ulReserved - must be zero
  flags: MapiFileFlags, // ULONG  flFlags - flags
  position: ULong, // ULONG  nPosition - character in text to be replaced by attachment
  pub path_name: LpStr, // LPSTR  lpszPathName - full path name of attachment file
  file_name: LpStr, // LPSTR  lpszFileName - original file name (optional)
  file_type: *const RawMapiFileTagExt // LPVOID lpFileType - attachment file type (can be lpMapiFileTagExt)
}

#[derive(Debug)]
pub struct FileDescriptor {
	flags: MapiFileFlags,
	position: ULong,
	path_name: String,
	file_name: Option<String>,
	file_type: Option<FileTagExtension>,
}

impl From<&RawMapiFileDesc> for FileDescriptor {
    fn from(raw: &RawMapiFileDesc) -> Self {
		let file_type_result = FileTagExtension::try_from(raw.file_type);
	    FileDescriptor {
			flags: raw.flags,
			position: raw.position,
			path_name: maybe_string_from_raw_ptr(raw.path_name).unwrap_or("MISSING_PATH".to_owned()),
			file_name: maybe_string_from_raw_ptr(raw.file_name),
			file_type: file_type_result.ok(),
		}
  	}
}


#[repr(C)]
#[derive(Debug)]
pub struct RawMapiRecipDesc {
	reserved: ULong, 			// ULONG ulReserved - reserved for future use
	recip_class: ULong,		// ULONG ulRecipClass - recipient class
	name: LpStr, // LPSTR lpszName - recipient name
	address: LpStr,// LPSTR lpszAddress - recitpient address (optional)
	eid_size: ULong,			// ULONG ulEIDSize count in bytes of size of pEntryID
	entry_id: *const libc::c_uchar,	// LPVOID lpEntryID system-specific recipient reference
}

#[derive(Debug)]
pub struct RecipientDescriptor {
	recip_class: ULong,
	name: String,
	address: Option<String>,
	entry_id: Vec<u8>,
}

impl TryFrom<*const RawMapiRecipDesc> for RecipientDescriptor {
	type Error = ();
	fn try_from(raw_ptr: *const RawMapiRecipDesc) -> Result<Self, Self::Error> {
		if std::ptr::null() == raw_ptr {
			Err(())
		} else {
			let raw : &RawMapiRecipDesc = unsafe {&*raw_ptr};
			Ok(Self::from(raw))
		}
	}
}

impl From<&RawMapiRecipDesc> for RecipientDescriptor {
	fn from(raw : &RawMapiRecipDesc) -> Self {
		RecipientDescriptor {
			recip_class: raw.recip_class,
			name: maybe_string_from_raw_ptr(raw.name).unwrap_or("MISSING_RECIP_NAME".to_owned()),
			address: maybe_string_from_raw_ptr(raw.address),
			entry_id: copy_c_array_to_vec(raw.entry_id, raw.eid_size as usize),			
		}
	}
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/ns-mapi-mapimessage
#[repr(C)]
#[derive(Debug)]
pub struct RawMapiMessage {
	reserved: ULong,			// ULONG ulReserved - reserved, must be 0 or CP_UTF8
	subject: LpStr, // LPSTR lpszSubject - message subject
	note_text: LpStr, // LPSTR lpszNoteText - message text
	message_type: LpStr, // LPSTR lpszMessageType - message class
	date_received: LpStr, // LPSTR lpszDateReceived - in YYYY/MM/DD HH:MM format
	conversation_id: LpStr, // LPSTR lpszConversationID - conversation thread id
	flags: MapiMessageFlags, // TODO: FLAGS flFlags - unread, return receipt
	originator: *const RawMapiRecipDesc, // TODO: lpMapiRecipDesc lpOriginator - originator descriptor
	recip_count: ULong,		// ULONG nRecipCount - number of recipients
	recips: *const RawMapiRecipDesc, 	// TODO: lpMapiRecipDesc lpRecips - recipient descriptors
	file_count: ULong,		// ULONG nFileCount - # of file attachments
	files: *const RawMapiFileDesc,	// TODO: lpMapiFileDesc lpFiles - attachment descriptors
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
			let raw = unsafe {&*raw_ptr};
			let originator_result = RecipientDescriptor::try_from(raw.originator);
			let recips : Vec<RecipientDescriptor> = raw_to_vec(raw.recips, raw.recip_count as usize);
			let files : Vec<FileDescriptor> = raw_to_vec(raw.files, raw.file_count as usize);
			Ok(Message {
				subject: maybe_string_from_raw_ptr(raw.subject),
				note_text: maybe_string_from_raw_ptr(raw.note_text),
				message_type: maybe_string_from_raw_ptr(raw.message_type),
				date_received: maybe_string_from_raw_ptr(raw.date_received),
				conversation_id: maybe_string_from_raw_ptr(raw.conversation_id),
				flags: raw.flags,
				originator: originator_result.ok(),
				recips,
				files,
			})
		}
	}
}

impl Message {
	pub fn make_mailto_link(&self) -> String {
		let to = self.recips.iter()
			.nth(1)
			.map(|r| r.address.clone())
			.unwrap_or(Some("".to_owned()));
		let cc = self.recips.iter()
			.skip(1)
			.filter_map(|r| r.address.clone())
			.collect::<Vec<String>>()
			.join(",");
		let subject = self.subject.as_ref()
			.map(|s| s.clone())
			.unwrap_or("".to_owned());
		let body = self.note_text.as_ref()
			.map(|s| s.clone())
			.unwrap_or("".to_owned());
			
		format!("mailto:{}?cc={}&bcc={}&subject={}&body={}", 
			to.unwrap(),
			cc,
			"".to_owned(), 
			encode(&subject), 
			encode(&body)
		)
	}

	pub fn make_attachment_arg(&self) -> String {
		"NOPE".to_owned()
	}
}
