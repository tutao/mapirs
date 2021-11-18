use std::convert::{From, TryFrom, TryInto};
use std::ffi::CStr;

use crate::types::LpStr;

/// get an owned String from a raw pointer if it is valid UTF8
/// copies the contents into a new buffer
pub fn maybe_string_from_raw_ptr(ptr: LpStr) -> Option<String> {
    if ptr.is_null() {
        return None;
    }

    // SAFETY: https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.from_ptr
    // * There is no guarantee to the validity of ptr:
    //   -> we checked that the ptr is not null, other invalid ptrs would be a bug in the calling app
    // * The returned lifetime is not guaranteed to be the actual lifetime of ptr:
    //   -> we immediately clone the resulting Cstr into a Heap-allocated String
    // * There is no guarantee that the memory pointed to by ptr contains a valid
    //   nul terminator byte at the end of the string:
    //   -> this would be a bug in the calling app
    // * It is not guaranteed that the memory pointed by ptr won't change before the CStr has been
    //   destroyed:
    //   -> this would be a bug in the calling app. We're only reading from the CStr once to clone
    //       it into an owned String, so the window for the memory to change is small.
    let maybe_str = unsafe { CStr::from_ptr(ptr) }.to_str();

    if let Ok(s) = maybe_str {
        // after this, we don't care about the memory pointed by ptr anymore
        Some(String::from(s))
    } else {
        None
    }
}

/// convert a raw pointer + an element count into a vec
/// by copying all the elements into a new vec and calling TryFrom on them
///
/// returns an empty vec if the pointer is invalid or count is 0
pub fn raw_to_vec<'a, K: TryFrom<&'a T>, T: 'a>(ptr: *const T, count: usize) -> Vec<Option<K>> {
    let mut v: Vec<Option<K>> = vec![];
    // the ptr must be aligned to T's alignment for this to be safe:
    // https://doc.rust-lang.org/reference/type-layout.html#reprc-structs
    // align_offset == 0 may have false negatives:
    // https://doc.rust-lang.org/std/primitive.pointer.html#method.align_offset
    if ptr.is_null() || count == 0 || ptr.align_offset(std::mem::align_of::<T>()) != 0 {
        return v;
    }
    /*
    SAFETY: https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html#safety
    checked for null, don't use the pointer at all when count is zero, checked alignment and
    don't mutate the pointee,
    so the remaining unsafety comes from
        * invalid T's
        * slice spans multiple allocations
        * size of slice exceeds isize::MAX

     all of these would be bugs in the calling application.
     */
    let slc: &[T] = unsafe { std::slice::from_raw_parts(ptr, count) };
    for t in slc.iter() {
        v.push(t.try_into().ok());
    }
    v
}

/// convert a raw pointer + an element count into a vec
/// by copying all the elements from that pointer on into a new vec
/// returns an empty vec if the pointer is invalid or count is 0
pub fn copy_c_array_to_vec<T: Clone>(ptr: *const T, count: usize) -> Vec<T> {
    if ptr.is_null() || count == 0 || ptr.align_offset(std::mem::align_of::<T>()) != 0 {
        vec![]
    } else {
        /*
        SAFETY: https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html#safety
        checked for null, don't use the pointer at all when count is zero, checked alignment and
        don't mutate the pointee,
        so the remaining unsafety comes from
            * invalid T's
            * slice spans multiple allocations
            * size of slice exceeds isize::MAX

         all of these would be bugs in the calling application.
         */
        let slc: &[T] = unsafe { std::slice::from_raw_parts(ptr, count) };
        // after this, we don't care about the pointee anymore since we own the buffer.
        slc.to_vec()
    }
}

/// MapiSendDocuments gets its file paths as a list packed into a string with
/// a delimiter:
/// C:\a.txt;C:\b.txt;A:\d.jpg
pub fn unpack_strings(packed: String, delim: &str) -> Vec<String> {
    match delim {
        "" => vec![packed],
        _ => packed
            .split(delim)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_owned())
            .collect(),
    }
}

#[cfg(test)]
mod test {
    use crate::ffi::conversion::unpack_strings;

    #[test]
    fn unpack_strings_works() {
        let delim = ";".to_owned();
        assert_eq!(
            unpack_strings("A;B;C".to_owned(), &delim),
            vec!["A", "B", "C"]
        );

        assert_eq!(unpack_strings("".to_owned(), &delim), Vec::<String>::new());

        assert_eq!(
            unpack_strings(";;".to_owned(), &delim),
            Vec::<String>::new()
        );

        assert_eq!(
            unpack_strings("C:\\a.txt;C:\\b.jpg".to_owned(), &"".to_owned()),
            vec!["C:\\a.txt;C:\\b.jpg"]
        );

        assert_eq!(
            unpack_strings("C:\\a.txt;C:\\b.jpg".to_owned(), &"%".to_owned()),
            vec!["C:\\a.txt;C:\\b.jpg"]
        );

        assert_eq!(
            unpack_strings(";C:\\a.txt;".to_owned(), &delim),
            vec!["C:\\a.txt"]
        );
    }
}
