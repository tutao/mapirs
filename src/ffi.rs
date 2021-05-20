use crate::structs::{
	RawMapiRecipDesc,
	RawMapiMessage,
	Message,
};

use crate::flags::{
	MapiStatusCode,
	MapiLogonFlags,
	MapiSendMailFlags,
	MapiFindNextFlags,
	MapiAddressFlags,
	MapiResolveNameFlags,
	MapiSaveMailFlags,
	MapiReadMailFlags,
	MapiDetailsFlags,
};

use std::convert::TryFrom;
use std::fs;
use std::fs::{
	OpenOptions,
	File,
};
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use directories::BaseDirs;

fn get_current_time() -> ::std::time::Duration {
	SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards")
}

fn log_to_file(caller: &str, stuff: &str) -> () {
	let base_dirs = BaseDirs::new().unwrap();
	let data_dir = base_dirs.data_dir();
	let logpath = data_dir.join("tutanota-desktop");
	let logfile = logpath.join("mapilog.txt");
	fs::create_dir_all(logpath).unwrap();
	let mut file = OpenOptions::new()
		.write(true)
		.append(true)
		.open(logfile.clone()) // PathBuf is not copy
		.or_else(|_|{File::create(logfile)})
		.unwrap();

	if let Err(e) = writeln!(file, "{} | {}: {}", get_current_time().as_millis(), caller, stuff) {
		eprintln!("Couldn't write to file: {}", e);
	}
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapilogon
#[no_mangle]
pub extern "C" fn MAPILogon(
	ui_param: *const u64, 				// ULONG_PTR ulUIParam
	profile_name: *const libc::c_char, 	// LPSTR lpszProfileName
	password: *const libc::c_char,		// LPSTR lpszPassword
	flags: MapiLogonFlags,				// FLAGS flFlags
	reserved: u64,						// ULONG ulReserved (mb 0)
	session: *const libc::c_void,		// TODO: LPLHANDLE lplhSession
) -> MapiStatusCode {
	log_to_file("mapilogon", "");
	MapiStatusCode::NotSupported
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapilogoff
#[no_mangle]
pub extern "C" fn MAPILogoff(
	session: *const libc::c_void, 		// TODO: LHANDLE lhSession
	ui_param: *const u64,				// ULONG_PTR ulUIParam
	flags: u64,							// FLAGS flFlags (reserved, must be zero)
	reserved: u64,						// ULONG ulReserved
) -> MapiStatusCode {
	log_to_file("mapilogoff", "");
	MapiStatusCode::NotSupported
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapisendmail
#[no_mangle]
pub extern "C" fn MAPISendMail(
	session: *const libc::c_void,		// TODO: LHANDLE lhSession
	ui_param: *const u64,
	message: *const RawMapiMessage,		// lpMapiMessage lpMessage
	flags: MapiSendMailFlags,		// FLAGS flFlags
	reserved: u64,
	
) -> MapiStatusCode {
	let msg = Message::try_from(message);
	let text = format!("session: {:?},\nui_param: {:?},\nmessage: {:?},\nflags: {:?}\n", session, ui_param, msg, flags);
	log_to_file("mapisendmail", &text);
	MapiStatusCode::NotSupported
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapisenddocuments
#[no_mangle]
pub extern "C" fn MAPISendDocuments(
	ui_param: *const u64,
	delim_char: *const libc::c_char,	// __in LPSTR lpszDelimChar
	file_paths: *const libc::c_char,	// __in LPSTR lpszFilePaths
	file_names: *const libc::c_char,	// __in LPSTR lpszFileNames
	reserved: u64,
) -> MapiStatusCode {
	log_to_file("mapisenddocuments", "");
	MapiStatusCode::NotSupported
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapifindnext
#[no_mangle]
pub extern "C" fn MAPIFindNext(
	session: *const libc::c_void,
	ui_param: *const u64,
	message_type: *const libc::c_char,		// LPSTR lpszMessageType
	seed_message_id: *const libc::c_char,	// LPSTR lpszSeedMessageID
	flags: MapiFindNextFlags,
	reserved: u64,							
	message_id: *const libc::c_char,		// LPSTR lpszMessageID
)-> MapiStatusCode {
	log_to_file("mapifindnext", "");
	MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPIReadMail(
	session: *const libc::c_void,
	ui_param: *const u64,
	message_id: *mut libc::c_char, // __in LPSTR lpszMessageID
	flags: MapiReadMailFlags,
	reserved: u64,
	message: *const RawMapiMessage, // lpMapiMessage FAR *lppMessage
) -> MapiStatusCode {
	log_to_file("mapireadmail", "");
	MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPISaveMail(
	session: *const libc::c_void,
	ui_param: *const u64,
	message: *const RawMapiMessage, // lpMapimessage lpMessage
	flags: MapiSaveMailFlags,
	reserved: u64,
	message_id: *mut libc::c_char, // __in LPSTR lpszMessageID
) -> MapiStatusCode {
	log_to_file("mapisavemail", "");
	MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPIDeleteMail(
	session: *const libc::c_void,
	ui_param: *const u64,
	message_id: *mut libc::c_char, // __in LPSTR lpsz MessageID
	flags: u64, // reserved, must be zero
	reserved: u64,
) -> MapiStatusCode {
	log_to_file("mapideletemail", "");
	MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPIFreeBuffer(
	pv: *const libc::c_void,
) -> MapiStatusCode {
	log_to_file("mapifreebuffer", "");
	MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPIAddress(
	session: *const libc::c_void,
	ui_param: *const u64,
	caption: *mut libc::c_char, // __in LPSTR lpszCaption
	edit_fields: u64,
	labels: *mut libc::c_char, // __in LPSTR lpszLabels
	n_recipients: u64,
	recipients: *const RawMapiRecipDesc, // lpMapiRecipDesc lpRecips
	flags: MapiAddressFlags,
	reserved: u64,
	n_new_recipients: *const u64, // LPULONG lpnNewRecips
	new_recipients: *const RawMapiRecipDesc, // lpMapiRecipDesc FAR *lppNewRecips
) -> MapiStatusCode {
	log_to_file("mapiaddress", "");
	MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPIDetails(
	session: *const libc::c_void,
	ui_param: *const u64,
	recipient: *const RawMapiRecipDesc,
	flags: MapiDetailsFlags,
	reserved: u64,
) -> MapiStatusCode {
	log_to_file("mapidetails", "");
	MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern  "C" fn MAPIResolveName(
	session: *const libc::c_void,
	ui_param: *const u64,
	name: *mut libc::c_char, // __in LPSTR lpszName
	flags: MapiResolveNameFlags,
	reserved: u64,
	recipient: *const RawMapiRecipDesc, // lpMapiRecipDesc FAR *lppRecip
) -> MapiStatusCode {
	log_to_file("mapiresolvename", "");
	MapiStatusCode::NotSupported
}

