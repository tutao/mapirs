use std::convert::TryFrom;
use std::convert::From;
use std::convert::Into;
use std::ffi::CStr;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::fs;
use std::io;
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
	/// absolute path to attachment
	path_name: PathBuf,
	/// file name to use for the attachment (if different from the name in the path)
	file_name: Option<PathBuf>,
	file_type: Option<FileTagExtension>,
}

impl From<&RawMapiFileDesc> for FileDescriptor {
    fn from(raw: &RawMapiFileDesc) -> Self {
		let file_type_result = FileTagExtension::try_from(raw.file_type);
	    FileDescriptor {
			flags: raw.flags,
			position: raw.position,
			path_name: maybe_string_from_raw_ptr(raw.path_name)
				.map(|s| PathBuf::from(s))
				.unwrap_or(PathBuf::from("INVALID_PATH")),
			file_name: maybe_string_from_raw_ptr(raw.file_name)
				.map(|s| PathBuf::from(s)),
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

	/// FileDescriptors may have a path and a separate file name
	/// to make it easier, copy the attachment to the file name
	/// next to the path.
	/// don't do anything if file name is the same as the path pointed to
	/// or if the attachment file descriptor doesn't have a separate file
	/// name.
	pub fn ensure_attachments(&self) -> io::Result<()> {
		for file_desc in &self.files {
			let maybe_path = swap_filename(&file_desc.path_name, &file_desc.file_name);
			let new_path = if let Some(np) = maybe_path {
				np
			} else {
				continue;
			};
			fs::copy(
				&file_desc.path_name,
				&new_path,
			)?;
		}
		Ok(())
	}

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
		let attachments = self.files.iter()
			.map(|fd| swap_filename(&fd.path_name, &fd.file_name).unwrap_or(fd.path_name.clone()));
			
		format!("mailto:{}?cc={}&subject={}&body={}", 
			to.unwrap(),
			cc,
			encode(&subject), 
			encode(&body)
		)
	}
}


/// get a path to a file in the same directory as file_path but named file_name
/// see tests for examples
fn swap_filename(file_path: &PathBuf, file_name: &Option<PathBuf>) -> Option<PathBuf> {
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
		return None;
	}

	Some(dir_path.join(file_name))
}

#[cfg(test)]
mod tests {
	use std::path::PathBuf;
	use crate::structs::*;
	
	#[test]
	fn swap_filename_works() {
		let fpath1 = PathBuf::from("/home/u/text.txt");
		let fpath2 = PathBuf::from("/");
		let fpath3 = PathBuf::from("/home/no/");
		let fname1 = Some(PathBuf::from("image.jpg"));
		let fname2 = None;
		let fname3 = Some(PathBuf::from("hello/image.jpg"));
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
	}
}
