use std::convert::TryFrom;

use crate::commands;
use crate::commands::send_mail;
use crate::ffi::conversion::{maybe_string_from_raw_ptr, unpack_strings};
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
    // ULONG_PTR ulUIParam
    _ui_param: ULongPtr,
    // LPSTR lpszProfileName
    _profile_name: LpStr,
    // LPSTR lpszPassword
    _password: LpStr,
    // FLAGS flFlags
    _flags: MapiLogonFlags,
    // ULONG ulReserved (mb 0)
    _reserved: ULong,
    // was LPLHANDLE lplhSession. fix if we ever want to use sessions
    _session: LpVoid,
) -> MapiStatusCode {
    commands::log_to_file("mapilogon", "");
    MapiStatusCode::NotSupported
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapilogoff
#[no_mangle]
pub extern "C" fn MAPILogoff(
    // was LPLHANDLE lplhSession. fix if we ever want to use sessions
    _session: LpVoid,
    // ULONG_PTR ulUIParam
    _ui_param: ULongPtr,
    // FLAGS flFlags (reserved, must be zero)
    _flags: ULong,
    // ULONG ulReserved
    _reserved: ULong,
) -> MapiStatusCode {
    commands::log_to_file("mapilogoff", "");
    MapiStatusCode::NotSupported
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapisendmail
#[no_mangle]
pub extern "C" fn MAPISendMail(
    // was LPLHANDLE lplhSession. fix if we ever want to use sessions
    _session: LpVoid,
    // ULONG_PTR
    _ui_param: ULongPtr,
    // lpMapiMessage lpMessage
    message: *const RawMapiMessage,
    // FLAGS flFlags
    _flags: MapiSendMailFlags,
    // ULONG reserved mb 0
    _reserved: ULong,
) -> MapiStatusCode {
    if let Ok(msg) = Message::try_from(message) {
        commands::log_to_file("mapisendmail", "parsed message, sending...");
        if let Err(e) = send_mail(msg) {
            commands::log_to_file("mapisendmail", &format!("could not send mail: {:?}", e));
            MapiStatusCode::Failure
        } else {
            commands::log_to_file("mapisendmail", "sent message!");
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
    // __in LPSTR lpszDelimChar
    delim_char: InLpStr,
    // __in LPSTR lpszFilePaths
    file_paths: InLpStr,
    // __in LPSTR lpszFileNames
    file_names: InLpStr,
    _reserved: ULong,
) -> MapiStatusCode {
    commands::log_to_file("mapisenddocuments", "");
    // some app may put null as delim if there's only one path
    let delim = maybe_string_from_raw_ptr(delim_char).unwrap_or_else(|| "".to_owned());

    // spec says if this is empty or null, show sendmail dialog without attachments
    let packed_paths = maybe_string_from_raw_ptr(file_paths).unwrap_or_else(|| "".to_owned());
    // spec says if this is empty or null, ignore
    let packed_names = maybe_string_from_raw_ptr(file_names).unwrap_or_else(|| "".to_owned());

    let paths = unpack_strings(packed_paths, &delim);
    let names = unpack_strings(packed_names, &delim);

    let msg = Message::from_paths(paths, names);

    commands::log_to_file("mapisenddocument", "parsed documents, sending...");
    if let Err(e) = send_mail(msg) {
        commands::log_to_file("mapisenddocument", &format!("could not send mail: {:?}", e));
        MapiStatusCode::Failure
    } else {
        commands::log_to_file("mapisenddocument", "sent message!");
        MapiStatusCode::Success
    }
}

/// https://docs.microsoft.com/en-us/windows/win32/api/mapi/nc-mapi-mapifindnext
#[no_mangle]
pub extern "C" fn MAPIFindNext(
    _session: LpVoid,
    _ui_param: ULongPtr,
    // LPSTR lpszMessageType
    _message_type: LpStr,
    // LPSTR lpszSeedMessageID
    _seed_message_id: LpStr,
    _flags: MapiFindNextFlags,
    _reserved: ULong,
    // LPSTR lpszMessageID
    _message_id: LpStr,
) -> MapiStatusCode {
    commands::log_to_file("mapifindnext", "");
    MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPIReadMail(
    _session: LpVoid,
    _ui_param: ULongPtr,
    // __in LPSTR lpszMessageID
    _message_id: InLpStr,
    _flags: MapiReadMailFlags,
    _reserved: ULong,
    // lpMapiMessage FAR *lppMessage
    _message: *const RawMapiMessage,
) -> MapiStatusCode {
    commands::log_to_file("mapireadmail", "");
    MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPISaveMail(
    _session: LpVoid,
    _ui_param: ULongPtr,
    // lpMapimessage lpMessage
    _message: *const RawMapiMessage,
    _flags: MapiSaveMailFlags,
    _reserved: ULong,
    // __in LPSTR lpszMessageID
    _message_id: InLpStr,
) -> MapiStatusCode {
    commands::log_to_file("mapisavemail", "");
    MapiStatusCode::NotSupported
}

#[no_mangle]
pub extern "C" fn MAPIDeleteMail(
    _session: LpVoid,
    _ui_param: ULongPtr,
    // __in LPSTR lpsz MessageID
    _message_id: InLpStr,
    // reserved, must be zero
    _flags: ULong,
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
    // ULONG_PTR
    _ui_param: ULongPtr,
    // __in LPSTR lpszCaption
    _caption: InLpStr,
    // ULONG
    _n_edit_fields: ULong,
    // __in LPSTR lpszLabels
    _labels: InLpStr,
    // ULONG
    _n_recipients: ULong,
    // lpMapiRecipDesc lpRecips
    _recipients: *const RawMapiRecipDesc,
    _flags: MapiAddressFlags,
    // ULONG
    _reserved: ULong,
    // LPULONG lpnNewRecips
    _n_new_recipients: ULongPtr,
    // lpMapiRecipDesc FAR *lppNewRecips
    _new_recipients: *const RawMapiRecipDesc,
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
    // __in LPSTR lpszName
    _name: InLpStr,
    _flags: MapiResolveNameFlags,
    _reserved: ULong,
    // lpMapiRecipDesc FAR *lppRecip
    _recipient: *const RawMapiRecipDesc,
) -> MapiStatusCode {
    commands::log_to_file("mapiresolvename", "");
    MapiStatusCode::NotSupported
}
