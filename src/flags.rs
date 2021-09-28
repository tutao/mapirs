use crate::types::*;

bitflags! {
    #[repr(C)]
    pub struct MapiFileFlags: ULong {
        const OLE 		= 0x00000001;
        const OLE_STATIC 	= 0x00000002;
    }
}

bitflags! {
    #[repr(C)]
    pub struct MapiMessageFlags: ULong {
        const UNREAD 		= 0x00000001;
        const RECEIPT_REQUESTED = 0x00000002;
        const SENT 		= 0x00000004;
    }
}

// ULONG are 32 bit on windows
#[repr(u32)]
#[allow(dead_code)]
pub enum MessageCodePage {
    Ansi = 0,
    Utf8 = 65001,
}

// ULONG
#[repr(u32)]
#[allow(dead_code)]
pub enum MapiRecipClass {
    Orig = 0,
    To = 1,
    Cc = 2,
    Bcc = 3,
}

// ULONG
#[repr(u32)]
pub enum MapiStatusCode {
    Success = 0,
    UserAbort = 1,
    Failure = 2,
    LogonFailure = 3,
    DiskFull = 4,
    InsufficientMemory = 5,
    AccessDenied = 6,
    TooManySessions = 8,
    TooManyFiles = 9,
    TooManyRecipients = 10,
    AttachmentNotFound = 11,
    AttachmentOpenFailure = 12,
    AttachmentWriteFailure = 13,
    UnknownRecipient = 14,
    BadRecipType = 15,
    NoMessages = 16,
    InvalidMessage = 17,
    TextTooLarge = 18,
    InvalidSession = 19,
    TypeNotSupported = 20,
    AmbiguousRecipient = 21,
    MessageInUse = 22,
    NetworkFailure = 23,
    InvalidEditfields = 24,
    InvalidRecips = 25,
    NotSupported = 26,
}

bitflags! {
    /// MAPILogon() flags
    #[repr(C)]
    pub struct MapiLogonFlags: ULong {
        /// display logon UI
        const LOGON_UI 			= 0x00000001;
        /// prompt for pw only
        const PASSWORD_UI 		= 0x00020000;
        /// don't use shared session
        const NEW_SESSION 		= 0x00000002;
        /// get new mail before return
        const FORCE_DOWNLOAD 		= 0x00001000;
        /// extended mapi logon
        const EXTENDED 			= 0x00000020;
    }
}

bitflags! {
    /// MAPISendMail() flags
    #[repr(C)]
    pub struct MapiSendMailFlags: ULong {
        const LOGON_UI			= MapiLogonFlags::LOGON_UI.bits();
        const NEW_SESSION		= MapiLogonFlags::NEW_SESSION.bits();
        /// display a send note UI
        const DIALOG 			= 0x00000008;
        /// use default profile in logon
        const MAPI_USE_DEFAULT  	= 0x00000040;
    }
}

bitflags! {
    /// MAPIFindNext() flags
    #[repr(C)]
    pub struct MapiFindNextFlags: ULong {
        /// only unread messages
        const UNREAD_ONLY 		= 0x00000020;
        /// use date order
        const GUARANTEE_FIFO 		= 0x00000100;
        /// allow 512 char returned ID
        const LONG_MSGID 		= 0x00004000;
    }
}

bitflags! {
    /// MAPIReadMail() flags
    #[repr(C)]
    pub struct MapiReadMailFlags: ULong {
        /// do not mark as read
        const PEEK 			= 0x00000080;
        /// header + body, no files
        const SUPPRESS_ATTACH  		= 0x00000800;
        /// only header info
        const ENVELOPE_ONLY		= 0x00000040;
        /// n/a
        const BODY_AS_FILE		= 0x00000200;
    }
}

bitflags! {
    /// MAPISaveMail() flags
    #[repr(C)]
    pub struct MapiSaveMailFlags: ULong {
        const LOGON_UI		= MapiLogonFlags::LOGON_UI.bits();
        const NEW_SESSION	= MapiLogonFlags::NEW_SESSION.bits();
        const LONG_MSGID	= MapiFindNextFlags::LONG_MSGID.bits();
    }
}

bitflags! {
    /// MAPIAddress() flags
    #[repr(C)]
    pub struct MapiAddressFlags: ULong {
        const LOGON_UI		= MapiLogonFlags::LOGON_UI.bits();
        const NEW_SESSION	= MapiLogonFlags::NEW_SESSION.bits();
    }
}

bitflags! {
    /// MAPIDetails() flags
    #[repr(C)]
    pub struct MapiDetailsFlags: ULong {
        const LOGON_UI		= MapiLogonFlags::LOGON_UI.bits();
        const NEW_SESSION	= MapiLogonFlags::NEW_SESSION.bits();
        /// don't allow mods of AB entries
        const AB_NOMODIFY 	= 0x00000400;
    }
}

bitflags! {
    /// MAPIResolveName() flags
    #[repr(C)]
    pub struct MapiResolveNameFlags: ULong {
        const LOGON_UI  		= MapiLogonFlags::LOGON_UI.bits();
        const NEW_SESSION 		= MapiLogonFlags::NEW_SESSION.bits();
        const DIALOG			= MapiSendMailFlags::DIALOG.bits();
        const AB_NOMODIFY		= MapiDetailsFlags::AB_NOMODIFY.bits();
    }
}
