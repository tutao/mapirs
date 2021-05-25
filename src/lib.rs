#![allow(dead_code)]
#![allow(unused_variables)]
#[macro_use]
extern crate bitflags;
extern crate directories;
extern crate winreg;

// the main data structures from MAPI.h and their safe counterparts
mod structs;
// flag & enum definitions from MAPI.h
mod flags;
// responsible for formatting the commands to the client
mod commands;
// responsible for finding out where the client is installed
mod environment;
// the external API surface exposed to windows
mod ffi;
pub use crate::ffi::{
	MAPILogon,
	MAPILogoff,
	MAPISendMail,
	MAPISendDocuments,
	MAPIFindNext,
	MAPIReadMail,
	MAPISaveMail,
	MAPIDeleteMail,
	MAPIFreeBuffer,
	MAPIAddress,
	MAPIDetails,
	MAPIResolveName,
};
