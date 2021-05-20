use std::convert::TryFrom;
use crate::flags::{
	MapiFileFlags,
	MapiMessageFlags,
};

fn maybe_string_from_raw_ptr(ptr: *const libc::c_char) -> Option<String> {
	if std::ptr::null() != ptr {
		Some("DOOHICKEY".to_owned()) // TODO
	} else {
		None
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct RawMapiFileTagExt {
	reserved: u64, 			// ULONG ulReserved - reserved, must be zero
	cb_tag: u64,			// ULONG cbTag - size in bytes of the value defined by the lpTag member.
	lp_tag: *const u8, 	 	// LPBYTE lpTag - X.400 OID for this attachment type
	cb_encoding: u64,		// ULONG cbEncoding - size in bytes of
	lp_encoding: *const u8,		// LPBYTE lpEncoding - X.400 OID for this attachment's encoding
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
			let tag_slice: &[u8] = unsafe { std::slice::from_raw_parts(raw.lp_tag, raw.cb_tag as usize) };
			let encoding_slice: &[u8]  = unsafe { std::slice::from_raw_parts(raw.lp_encoding, raw.cb_encoding as usize)};
			Ok(FileTagExtension {
				tag: tag_slice.to_vec(),
				encoding: encoding_slice.to_vec(),
			})
		}
	}
}


#[repr(C)]
#[derive(Debug)]
pub struct RawMapiFileDesc {
  reserved: u64, // ULONG  ulReserved - must be zero
  flags: MapiFileFlags, // ULONG  flFlags - flags
  position: u64, // ULONG  nPosition - character in text to be replaced by attachment
  path_name: *const libc::c_char, // LPSTR  lpszPathName - full path name of attachment file
  file_name: *const libc::c_char, // LPSTR  lpszFileName - original file name (optional)
  file_type: *const RawMapiFileTagExt // LPVOID lpFileType - attachment file type (can be lpMapiFileTagExt)
}

#[derive(Debug)]
pub struct FileDescriptor {
	flags: MapiFileFlags,
	position: u64,
	path_name: String,
	file_name: Option<String>,
	file_type: Option<FileTagExtension>,
}

impl From<RawMapiFileDesc> for FileDescriptor {
    fn from(raw: RawMapiFileDesc) -> Self {
	let file_type_result = FileTagExtension::try_from(raw.file_type);
        FileDescriptor {
		flags: raw.flags,
		position: raw.position,
		path_name: "PATHNAME HERE".to_owned(), // TODO
		file_name: maybe_string_from_raw_ptr(raw.file_name),
		file_type: file_type_result.ok(),
	}
    }
}


#[repr(C)]
#[derive(Debug)]
pub struct RawMapiRecipDesc {
	reserved: u64, 			// ULONG ulReserved - reserved for future use
	recip_class: u64,		// ULONG ulRecipClass - recipient class
	name: *const libc::c_char, // LPSTR lpszName - recipient name
	address: *const libc::c_char,// LPSTR lpszAddress - recitpient address (optional)
	eid_size: u64,			// ULONG ulEIDSize count in bytes of size of pEntryID
	entry_id: *const libc::c_uchar,	// LPVOID lpEntryID system-specific recipient reference
}

#[derive(Debug)]
pub struct RecipientDescriptor {
	recip_class: u64,
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
			let raw = unsafe {&*raw_ptr};
			let entry_id_slice: &[u8] = unsafe { std::slice::from_raw_parts(raw.entry_id, raw.eid_size as usize) };

			Ok(RecipientDescriptor {
				recip_class: raw.recip_class,
				name: "RECIPIENT_NAME_HERE".to_owned(),	// TODO
				address: maybe_string_from_raw_ptr(raw.address),
				entry_id: entry_id_slice.to_vec(),			
			})
		}
	}
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/ns-mapi-mapimessage
#[repr(C)]
#[derive(Debug)]
pub struct RawMapiMessage {
	reserved: u64,			// ULONG ulReserved - reserved, must be 0 or CP_UTF8
	subject: *const libc::c_char, // LPSTR lpszSubject - message subject
	note_text: *const libc::c_char, // LPSTR lpszNoteText - message text
	message_type: *const libc::c_char, // LPSTR lpszMessageType - message class
	date_received: *const libc::c_char, // LPSTR lpszDateReceived - in YYYY/MM/DD HH:MM format
	conversation_id: *const libc::c_char, // LPSTR lpszConversationID - conversation thread id
	flags: MapiMessageFlags, // TODO: FLAGS flFlags - unread, return receipt
	originator: *const RawMapiRecipDesc, // TODO: lpMapiRecipDesc lpOriginator - originator descriptor
	recip_count: u64,		// ULONG nRecipCount - number of recipients
	recips: *const RawMapiRecipDesc, 	// TODO: lpMapiRecipDesc lpRecips - recipient descriptors
	file_count: u64,		// ULONG nFileCount - # of file attachments
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
			let recips = vec![];
			let files = vec![];
				
			Ok(Message {
				subject: maybe_string_from_raw_ptr(raw.subject),
				note_text: maybe_string_from_raw_ptr(raw.note_text),
				message_type: maybe_string_from_raw_ptr(raw.message_type),
				date_received: maybe_string_from_raw_ptr(raw.date_received),
				conversation_id: maybe_string_from_raw_ptr(raw.conversation_id),
				flags: raw.flags,
				originator: originator_result.ok(),
				recips: recips,
				files: files,
			})
		}
	}
}


