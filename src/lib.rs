#![allow(dead_code)]
#![allow(unused_variables)]
#[macro_use]
extern crate bitflags;
extern crate directories;

mod ffi;
mod structs;
mod flags;

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
