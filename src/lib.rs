#[macro_use]
extern crate bitflags;
mod flags;

use std::fs::File;
use std::io::prelude::*;
use crate::flags::{
	MapiFileFlags,
	MapiMessageFlags,
	MapiStatusCode,
	MapiLogonFlags,
	MapiSendMailFlags,
	MapiFindNextFlags,
};

#[repr(C)]
struct MapiFileDesc {
  reserved: u64, // ULONG  ulReserved - must be zero
  flags: MapiFileFlags, // ULONG  flFlags - flags
  position: u64, // ULONG  nPosition - character in text to be replaced by attachment
  path_name: *const libc::c_char, // LPSTR  lpszPathName - full path name of attachment file
  file_name: *const libc::c_char, // LPSTR  lpszFileName - original file name (optional)
  file_type: *const libc::c_void// LPVOID lpFileType - attachment file type (can be lpMapiFileTagExt)
}

#[repr(C)]
pub struct MapiFileTagExt {
	reserved: u64, 			// ULONG ulReserved - reserved, must be zero
	cb_tag: u64,			// ULONG cbTag - size in bytes of
	lp_tag: *const u8, 	 	// LPBYTE lpTag - X.400 OID for this attachment type
	cb_encoding: u64,		// ULONG cbEncoding - size in bytes of
	lp_encoding: *const u8,	// LPBYTE lpEncoding - X.400 OID for this attachment's encoding
}

#[repr(C)]
pub struct MapiRecipDesc {
	reserved: u64, 			// ULONG ulReserved - reserved for future use
	recip_class: u64,		// ULONG ulRecipClass - recipient class
	name: *const libc::c_char, // LPSTR lpszName - recipient name
	address: *const libc::c_char,// LPSTR lpszAddress - recitpient address (optional)
	eid_size: u64,			// ULONG ulEIDSize count in bytes of size of pEntryID
	entry_id: libc::c_void,	// LPVOID lpEntryID system-specific recipient reference
}

#[repr(C)]
pub struct MapiMessage {
	reserved: u64,			// ULONG ulReserved - reserved, must be 0
	subject: *const libc::c_char, // LPSTR lpszSubject - message subject
	note_text: *const libc::c_char, // LPSTR lpszNoteText - message text
	message_type: *const libc::c_char, // LPSTR lpszMessageType - message class
	date_received: *const libc::c_char, // LPSTR lpszDateReceived - in YYYY/MM/DD HH:MM format
	conversation_id: *const libc::c_char, // LPSTR lpszConversationID - conversation thread id
	flags: MapiMessageFlags, // TODO: FLAGS flFlags - unread, return receipt
	originator: *const MapiRecipDesc, // TODO: lpMapiRecipDesc lpOriginator - originator descriptor
	recip_count: u64,		// ULONG nRecipCount - number of recipients
	recips: *const MapiRecipDesc, 	// TODO: lpMapiRecipDesc lpRecips - recipient descriptors
	file_count: u64,		// ULONG nFileCount - # of file attachments
	files: *const MapiFileDesc,	// TODO: lpMapiFileDesc lpFiles - attachment descriptors
}


/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapilogon
#[no_mangle]
pub extern "C" fn MAPILOGON(
	ui_param: *const u64, 				// ULONG_PTR ulUIParam
	profile_name: *const libc::c_char, 	// LPSTR lpszProfileName
	password: *const libc::c_char,		// LPSTR lpszPassword
	flags: MapiLogonFlags,				// FLAGS flFlags
	reserved: u64,						// ULONG ulReserved (mb 0)
	session: *const libc::c_void,		// TODO: LPLHANDLE lplhSession
) -> MapiStatusCode {			
    MapiStatusCode::SUCCESS
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapilogoff
#[no_mangle]
pub extern "C" fn MAPILOGOFF(
	session: *const libc::c_void, 		// TODO: LHANDLE lhSession
	ui_param: *const u64,				// ULONG_PTR ulUIParam
	flags: u64,							// FLAGS flFlags (reserved, must be zero)
	reserved: u64,						// ULONG ulReserved
) -> MapiStatusCode {
	MapiStatusCode::SUCCESS
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapisendmail
#[no_mangle]
pub extern "C" fn MAPISENDMAIL(
	session: *const libc::c_void,		// TODO: LHANDLE lhSession
	ui_param: *const u64,
	message: *const MapiMessage,		// lpMapiMessage lpMessage
	flags: MapiSendMailFlags,			// FLAGS flFlags
	reserved: u64,
	
) -> MapiStatusCode {
	MapiStatusCode::SUCCESS
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapisenddocuments
#[no_mangle]
pub extern "C" fn MAPISENDDOCUMENTS(
	ui_param: *const u64,
	delim_char: *const libc::c_char,	// __in LPSTR lpszDelimChar
	file_paths: *const libc::c_char,	// __in LPSTR lpszFilePaths
	file_names: *const libc::c_char,	// __in LPSTR lpszFileNames
	reserved: u64,
) -> MapiStatusCode {
	MapiStatusCode::SUCCESS
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapifindnext
#[no_mangle]
pub extern "C" fn MAPIFINDNEXT(
	session: *const libc::c_void,
	ui_param: *const u64,
	message_type: *const libc::c_char,		// LPSTR lpszMessageType
	seed_message_id: *const libc::c_char,	// LPSTR lpszSeedMessageID
	flags: MapiFindNextFlags,
	reserved: u64,							
	message_id: *const libc::c_char,		// LPSTR lpszMessageID
)-> MapiStatusCode {
	MapiStatusCode::SUCCESS
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
