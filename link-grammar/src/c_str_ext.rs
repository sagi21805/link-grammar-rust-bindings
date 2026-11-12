//! Generic extensions for C strings.

use std::ffi::{CStr, c_char};

use extend::ext;

#[ext(name = CStrPtr)]
pub(crate) impl *const c_char {
    /// The UTF-8 string this pointer points to.
    ///
    /// # Safety
    ///
    /// The pointer is non-null, points to a NUL-terminated string, and the
    /// string stays alive and unchanged for `'a`.
    unsafe fn as_utf8<'a>(self) -> &'a str {
        debug_assert!(!self.is_null());
        // SAFETY: The caller guarantees a live NUL-terminated string.
        unsafe { CStr::from_ptr(self) }
            .to_str()
            .expect("link-grammar keeps every string in UTF-8")
    }
}
