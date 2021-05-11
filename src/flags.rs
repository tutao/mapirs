bitflags! {
	#[repr(C)]
    pub struct MapiFileFlags: u64 {
        const OLE 			= 0x00000001;
        const OLE_STATIC 	= 0x00000002;
    }
}

bitflags! {
	#[repr(C)]
	pub struct MapiMessageFlags: u64 {
		const UNREAD 			= 0x00000001;
		const RECEIPT_REQUESTED = 0x00000002;
		const SENT 				= 0x00000004;
	}	
}

#[repr(u64)]
pub enum MapiRecipClass {
	ORIG = 0,
	TO = 1,
	CC = 2,
	BCC = 3,
}

#[repr(u64)]
pub enum MapiStatusCode {
	SUCCESS 					= 0,
	USER_ABORT 					= 1,
	FAILURE 					= 2,
	LOGON_FAILURE 				= 3,
	DISK_FULL 					= 4,
	INSUFFICIENT_MEMORY 		= 5,
	ACCESS_DENIED 				= 6,
	TOO_MANY_SESSIONS 			= 8,
	TOO_MANY_FILES 				= 9,
	TOO_MANY_RECIPIENTS 		= 10,
	ATTACHMENT_NOT_FOUND 		= 11,
	ATTACHMENT_OPEN_FAILURE 	= 12,
	ATTACHMENT_WRITE_FAILURE 	= 13,
	UNKNOWN_RECIPIENT 			= 14,
	BAD_RECIPTYPE 				= 15,
	NO_MESSAGES 				= 16,
	INVALID_MESSAGE 			= 17,
	TEXT_TOO_LARGE 				= 18,
	INVALID_SESSION 			= 19,
	TYPE_NOT_SUPPORTED 			= 20,
	AMBIGUOUS_RECIPIENT 		= 21,
	MESSAGE_IN_USE 				= 22,
	NETWORK_FAILURE 			= 23,
	INVALID_EDITFIELDS 			= 24,
	INVALID_RECIPS 				= 25,
	NOT_SUPPORTED 				= 26
}


/// MAPILogon() flags
bitflags! {
	#[repr(C)]
	pub struct MapiLogonFlags: u64 {
		/// display logon UI
		const LOGON_UI 			= 0x00000001;
		/// prompt for pw only
		const PASSWORD_UI 		= 0x00020000;
		/// don't use shared session
		const NEW_SESSION 		= 0x00000002;
		/// get new mail before return 
		const FORCE_DOWNLOAD 	= 0x00001000;
		/// extended mapi logon
		const EXTENDED 			= 0x00000020;
	}
}

/// MAPISendMail() flags
bitflags! {
	#[repr(C)]
	pub struct MapiSendMailFlags: u64 {
		const LOGON_UI			= MapiLogonFlags::LOGON_UI.bits();
		const NEW_SESSION		= MapiLogonFlags::NEW_SESSION.bits();
		/// display a send note UI
		const DIALOG 			= 0x00000008;
		/// use default profile in logon
		const MAPI_USE_DEFAULT  = 0x00000040;
	}
}

/// MAPIFindNext() flags
bitflags! {
	#[repr(C)]
	pub struct MapiFindNextFlags: u64 {
		/// only unread messages
		const UNREAD_ONLY 		= 0x00000020; 
		/// use date order
		const GUARANTEE_FIFO 	= 0x00000100;
		/// allow 512 char returned ID
		const LONG_MSGID 		= 0x00004000;
	}
}

/// MAPIReadMail() flags
bitflags! {
	#[repr(C)]
	pub struct MapiReadMailFlags: u64 {
		/// do not mark as read
		const PEEK 				= 0x00000080;
		/// header + body, no files
		const SUPPRESS_ATTACH  	= 0x00000800;
		/// only header info
		const ENVELOPE_ONLY		= 0x00000040;
		/// n/a
		const BODY_AS_FILE		= 0x00000200;
	}
}

/// MAPISaveMail() flags
bitflags! {
	#[repr(C)]
	pub struct MapiSaveMailFlags: u64 {
		const LOGON_UI		= MapiLogonFlags::LOGON_UI.bits();
		const NEW_SESSION	= MapiLogonFlags::NEW_SESSION.bits();
		const LONG_MSGID	= MapiFindNextFlags::LONG_MSGID.bits();
	}
}

/// MAPIAddress() flags
bitflags! {
	#[repr(C)]
	pub struct MapiAddressFlags: u64 {
		const LOGON_UI		= MapiLogonFlags::LOGON_UI.bits();
		const NEW_SESSION	= MapiLogonFlags::NEW_SESSION.bits();
	}
}

/// MAPIDetails() flags 
bitflags! {
	#[repr(C)]
	pub struct MapiDetailsFlags: u64 {
		const LOGON_UI		= MapiLogonFlags::LOGON_UI.bits();
		const NEW_SESSION	= MapiLogonFlags::NEW_SESSION.bits();
		/// don't allow mods of AB entries
		const AB_NOMODIFY 	= 0x00000400;
	}
}

/// MAPIResolveName() flags
bitflags! {
	#[repr(C)]
	pub struct MapiResolveNameFlags: u64 {
		const LOGON_UI  		= MapiLogonFlags::LOGON_UI.bits();
		const NEW_SESSION 		= MapiLogonFlags::NEW_SESSION.bits();
		const DIALOG			= MapiSendMailFlags::DIALOG.bits();
		const AB_NOMODIFY		= MapiDetailsFlags::AB_NOMODIFY.bits();
	}
}
