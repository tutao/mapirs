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
    ui_param: ULongPtr,    // ULONG_PTR ulUIParam
    profile_name: LpStr,   // LPSTR lpszProfileName
    password: LpStr,       // LPSTR lpszPassword
    flags: MapiLogonFlags, // FLAGS flFlags
    reserved: ULong,       // ULONG ulReserved (mb 0)
    session: LpVoid,       // TODO: LPLHANDLE lplhSession??
) -> MapiStatusCode {
    commands::log_to_file("mapilogon", "");
    MapiStatusCode::NotSupported
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapilogoff
#[no_mangle]
pub extern "C" fn MAPILogoff(
    session: LpVoid,    // TODO: LHANDLE lhSession
    ui_param: ULongPtr, // ULONG_PTR ulUIParam
    flags: ULong,       // FLAGS flFlags (reserved, must be zero)
    reserved: ULong,    // ULONG ulReserved
) -> MapiStatusCode {
    commands::log_to_file("mapilogoff", "");
    MapiStatusCode::NotSupported
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapisendmail
#[no_mangle]
pub extern "C" fn MAPISendMail(
    session: LpVoid,                // TODO: LHANDLE lhSession
    ui_param: ULongPtr,             // ULONG_PTR
    message: *const RawMapiMessage, // lpMapiMessage lpMessage
    flags: MapiSendMailFlags,       // FLAGS flFlags
    reserved: ULong,                // ULONG reserved mb 0
) -> MapiStatusCode {
    if let Ok(msg) = Message::try_from(message) {
        send_mail(&msg);
        let text = format!("message: {:?}", msg);
        commands::log_to_file("mapisendmail", &text);
        MapiStatusCode::Success
    } else {
        MapiStatusCode::Failure
    }
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapisenddocuments
#[no_mangle]
pub extern "C" fn MAPISendDocuments(
    ui_param: ULongPtr,
    delim_char: InLpStr, // __in LPSTR lpszDelimChar
    file_paths: InLpStr, // __in LPSTR lpszFilePaths
    file_names: InLpStr, // __in LPSTR lpszFileNames
    reserved: ULong,
) -> MapiStatusCode {
    commands::log_to_file("mapisenddocuments", "");
    MapiStatusCode::NotSupported
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapifindnext
#[no_mangle]
pub extern "C" fn MAPIFindNext(
    session: LpVoid,
    ui_param: ULongPtr,
    message_type: LpStr,    // LPSTR lpszMessageType
    seed_message_id: LpStr, // LPSTR lpszSeedMessageID
    flags: MapiFindNextFlags,
    reserved: ULong,
    message_id: LpStr, // LPSTR lpszMessageID
) -> MapiStatusCode {
    commands::log_to_file("mapifindnext", "");
    MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPIReadMail(
    session: LpVoid,
    ui_param: ULongPtr,
    message_id: InLpStr, // __in LPSTR lpszMessageID
    flags: MapiReadMailFlags,
    reserved: ULong,
    message: *const RawMapiMessage, // lpMapiMessage FAR *lppMessage
) -> MapiStatusCode {
    commands::log_to_file("mapireadmail", "");
    MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPISaveMail(
    session: LpVoid,
    ui_param: ULongPtr,
    message: *const RawMapiMessage, // lpMapimessage lpMessage
    flags: MapiSaveMailFlags,
    reserved: ULong,
    message_id: InLpStr, // __in LPSTR lpszMessageID
) -> MapiStatusCode {
    commands::log_to_file("mapisavemail", "");
    MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPIDeleteMail(
    session: LpVoid,
    ui_param: ULongPtr,
    message_id: InLpStr, // __in LPSTR lpsz MessageID
    flags: ULong,        // reserved, must be zero
    reserved: ULong,
) -> MapiStatusCode {
    commands::log_to_file("mapideletemail", "");
    MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPIFreeBuffer(pv: LpVoid) -> MapiStatusCode {
    commands::log_to_file("mapifreebuffer", "");
    MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPIAddress(
    session: LpVoid,
    ui_param: ULongPtr,                  // ULONG_PTR
    caption: InLpStr,                    // __in LPSTR lpszCaption
    n_edit_fields: ULong,                // ULONG
    labels: InLpStr,                     // __in LPSTR lpszLabels
    n_recipients: ULong,                 // ULONG
    recipients: *const RawMapiRecipDesc, // lpMapiRecipDesc lpRecips
    flags: MapiAddressFlags,
    reserved: ULong,                         // ULONG
    n_new_recipients: ULongPtr,              // LPULONG lpnNewRecips
    new_recipients: *const RawMapiRecipDesc, // lpMapiRecipDesc FAR *lppNewRecips
) -> MapiStatusCode {
    commands::log_to_file("mapiaddress", "");
    MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPIDetails(
    session: LpVoid,
    ui_param: ULongPtr,
    recipient: *const RawMapiRecipDesc,
    flags: MapiDetailsFlags,
    reserved: ULong,
) -> MapiStatusCode {
    commands::log_to_file("mapidetails", "");
    MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPIResolveName(
    session: LpVoid,
    ui_param: ULong,
    name: InLpStr, // __in LPSTR lpszName
    flags: MapiResolveNameFlags,
    reserved: ULong,
    recipient: *const RawMapiRecipDesc, // lpMapiRecipDesc FAR *lppRecip
) -> MapiStatusCode {
    commands::log_to_file("mapiresolvename", "");
    MapiStatusCode::NotSupported
}
