use std::convert::TryFrom;

use crate::commands;
use crate::commands::send_mail;
use crate::flags::{
    MapiAddressFlags, MapiDetailsFlags, MapiFindNextFlags, MapiLogonFlags, MapiReadMailFlags,
    MapiResolveNameFlags, MapiSaveMailFlags, MapiSendMailFlags, MapiStatusCode,
};
use crate::structs::{Message, RawMapiMessage, RawMapiRecipDesc};
use crate::types::*;

pub mod conversion;

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapilogon
#[no_mangle]
pub extern "C" fn MAPILogon(
    _ui_param: ULongPtr,    // ULONG_PTR ulUIParam
    _profile_name: LpStr,   // LPSTR lpszProfileName
    _password: LpStr,       // LPSTR lpszPassword
    _flags: MapiLogonFlags, // FLAGS flFlags
    _reserved: ULong,       // ULONG ulReserved (mb 0)
    _session: LpVoid,       // was LPLHANDLE lplhSession. fix if we ever want to use sessions
) -> MapiStatusCode {
    commands::log_to_file("mapilogon", "");
    MapiStatusCode::NotSupported
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapilogoff
#[no_mangle]
pub extern "C" fn MAPILogoff(
    _session: LpVoid,    // was LPLHANDLE lplhSession. fix if we ever want to use sessions
    _ui_param: ULongPtr, // ULONG_PTR ulUIParam
    _flags: ULong,       // FLAGS flFlags (reserved, must be zero)
    _reserved: ULong,    // ULONG ulReserved
) -> MapiStatusCode {
    commands::log_to_file("mapilogoff", "");
    MapiStatusCode::NotSupported
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapisendmail
#[no_mangle]
pub extern "C" fn MAPISendMail(
    _session: LpVoid,                // was LPLHANDLE lplhSession. fix if we ever want to use sessions
    _ui_param: ULongPtr,             // ULONG_PTR
    message: *const RawMapiMessage, // lpMapiMessage lpMessage
    _flags: MapiSendMailFlags,       // FLAGS flFlags
    _reserved: ULong,                // ULONG reserved mb 0
) -> MapiStatusCode {
    if let Ok(msg) = Message::try_from(message) {
        commands::log_to_file("mapisendmail", "parsed message, sending...");
        if let Err(e) = send_mail(msg) {
            commands::log_to_file(
                "mapisendmail",
                &format!("could not send mail: {:?}", e),
            );
            MapiStatusCode::Failure
        } else {
            commands::log_to_file(
                "mapisendmail",
                "sent message!",
            );
            MapiStatusCode::Success
        }
    } else {
        commands::log_to_file("mapisendmail", "could not parse arguments.");
        MapiStatusCode::Failure
    }
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapisenddocuments
#[no_mangle]
pub extern "C" fn MAPISendDocuments(
    _ui_param: ULongPtr,
    _delim_char: InLpStr, // __in LPSTR lpszDelimChar
    _file_paths: InLpStr, // __in LPSTR lpszFilePaths
    _file_names: InLpStr, // __in LPSTR lpszFileNames
    _reserved: ULong,
) -> MapiStatusCode {
    commands::log_to_file("mapisenddocuments", "");
    MapiStatusCode::NotSupported
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapifindnext
#[no_mangle]
pub extern "C" fn MAPIFindNext(
    _session: LpVoid,
    _ui_param: ULongPtr,
    _message_type: LpStr,    // LPSTR lpszMessageType
    _seed_message_id: LpStr, // LPSTR lpszSeedMessageID
    _flags: MapiFindNextFlags,
    _reserved: ULong,
    _message_id: LpStr, // LPSTR lpszMessageID
) -> MapiStatusCode {
    commands::log_to_file("mapifindnext", "");
    MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPIReadMail(
    _session: LpVoid,
    _ui_param: ULongPtr,
    _message_id: InLpStr, // __in LPSTR lpszMessageID
    _flags: MapiReadMailFlags,
    _reserved: ULong,
    _message: *const RawMapiMessage, // lpMapiMessage FAR *lppMessage
) -> MapiStatusCode {
    commands::log_to_file("mapireadmail", "");
    MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPISaveMail(
    _session: LpVoid,
    _ui_param: ULongPtr,
    _message: *const RawMapiMessage, // lpMapimessage lpMessage
    _flags: MapiSaveMailFlags,
    _reserved: ULong,
    _message_id: InLpStr, // __in LPSTR lpszMessageID
) -> MapiStatusCode {
    commands::log_to_file("mapisavemail", "");
    MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPIDeleteMail(
    _session: LpVoid,
    _ui_param: ULongPtr,
    _message_id: InLpStr, // __in LPSTR lpsz MessageID
    _flags: ULong,        // reserved, must be zero
    _reserved: ULong,
) -> MapiStatusCode {
    commands::log_to_file("mapideletemail", "");
    MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPIFreeBuffer(_pv: LpVoid) -> MapiStatusCode {
    commands::log_to_file("mapifreebuffer", "");
    MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPIAddress(
    _session: LpVoid,
    _ui_param: ULongPtr,                  // ULONG_PTR
    _caption: InLpStr,                    // __in LPSTR lpszCaption
    _n_edit_fields: ULong,                // ULONG
    _labels: InLpStr,                     // __in LPSTR lpszLabels
    _n_recipients: ULong,                 // ULONG
    _recipients: *const RawMapiRecipDesc, // lpMapiRecipDesc lpRecips
    _flags: MapiAddressFlags,
    _reserved: ULong,                         // ULONG
    _n_new_recipients: ULongPtr,              // LPULONG lpnNewRecips
    _new_recipients: *const RawMapiRecipDesc, // lpMapiRecipDesc FAR *lppNewRecips
) -> MapiStatusCode {
    commands::log_to_file("mapiaddress", "");
    MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPIDetails(
    _session: LpVoid,
    _ui_param: ULongPtr,
    _recipient: *const RawMapiRecipDesc,
    _flags: MapiDetailsFlags,
    _reserved: ULong,
) -> MapiStatusCode {
    commands::log_to_file("mapidetails", "");
    MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPIResolveName(
    _session: LpVoid,
    _ui_param: ULong,
    _name: InLpStr, // __in LPSTR lpszName
    _flags: MapiResolveNameFlags,
    _reserved: ULong,
    _recipient: *const RawMapiRecipDesc, // lpMapiRecipDesc FAR *lppRecip
) -> MapiStatusCode {
    commands::log_to_file("mapiresolvename", "");
    MapiStatusCode::NotSupported
}
