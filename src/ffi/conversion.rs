use std::ffi::CStr;

use crate::types::LpStr;

/// get an owned String from a raw pointer if it is valid UTF8
/// copies the contents into a new buffer
pub fn maybe_string_from_raw_ptr(ptr: LpStr) -> Option<String> {
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

/// convert a raw pointer + an element count into a vec
/// by copying all the elements into a new vec and calling From on them
///
/// returns an empty vec if the pointer is invalid or count is 0
pub fn raw_to_vec<'a, K: From<&'a T>, T: 'a>(ptr: *const T, count: usize) -> Vec<K> {
    let mut v: Vec<K> = vec![];
    if std::ptr::null() == ptr || count == 0 {
        return v;
    }
    let slc: &[T] = unsafe { std::slice::from_raw_parts(ptr, count) };
    for t in slc.iter() {
        v.push(t.into());
    }
    v
}

/// convert a raw pointer + an element count into a vec
/// by copying all the elements from that pointer on into a new vec
/// returns an empty vec if the pointer is invalid or count is 0
pub fn copy_c_array_to_vec<T: Clone>(ptr: *const T, count: usize) -> Vec<T> {
    if std::ptr::null() == ptr || count == 0 {
        vec![]
    } else {
        let slc: &[T] = unsafe { std::slice::from_raw_parts(ptr, count) };
        slc.to_vec()
    }
}
