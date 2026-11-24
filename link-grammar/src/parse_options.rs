use std::ffi::c_int;
use std::ptr::NonNull;
use std::sync::MutexGuard;

use link_grammar_sys as sys;

use crate::verbosity::{self, Verbosity};

/// link-grammar's options for one parse, at the level
/// [`set_verbosity`](crate::set_verbosity) chose.
pub(crate) struct ParseOptions {
    raw: NonNull<sys::Parse_Options_s>,
}

impl ParseOptions {
    /// link-grammar's default options.
    pub(crate) fn new() -> Self {
        Self::with_verbosity(&verbosity::current())
    }

    /// Default options at `verbosity`, which also becomes link-grammar's
    /// global level.
    pub(crate) fn with_verbosity(
        verbosity: &MutexGuard<'_, Verbosity>,
    ) -> Self {
        // SAFETY: `parse_options_create` takes no arguments.
        let raw = unsafe { sys::parse_options_create() };
        let raw = NonNull::new(raw)
            .expect("parse_options_create only fails when malloc does");
        unsafe {
            sys::parse_options_set_verbosity(
                raw.as_ptr(),
                **verbosity as c_int,
            )
        };
        Self { raw }
    }

    /// The fewest words a linkage may leave unlinked.
    pub(crate) fn set_min_null_count(&mut self, count: usize) {
        unsafe {
            sys::parse_options_set_min_null_count(
                self.as_raw(),
                saturating_c_int(count),
            )
        };
    }

    /// The most words a linkage may leave unlinked.
    pub(crate) fn set_max_null_count(&mut self, count: usize) {
        unsafe {
            sys::parse_options_set_max_null_count(
                self.as_raw(),
                saturating_c_int(count),
            )
        };
    }

    /// The handle the C API takes.
    pub(crate) fn as_raw(&self) -> sys::Parse_Options { self.raw.as_ptr() }
}

impl Drop for ParseOptions {
    fn drop(&mut self) {
        // SAFETY: `raw` came from `parse_options_create`.
        unsafe { sys::parse_options_delete(self.raw.as_ptr()) };
    }
}

/// `count` as the C API's `int`, saturating at `c_int::MAX`.
fn saturating_c_int(count: usize) -> c_int {
    c_int::try_from(count).unwrap_or(c_int::MAX)
}
