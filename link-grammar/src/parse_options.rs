use std::ffi::c_int;
use std::ptr::NonNull;
use std::sync::MutexGuard;

use link_grammar_sys as sys;

use crate::verbosity::{self, Verbosity};

/// Settings that control how
/// [`Sentence::parse_with`](crate::Sentence::parse_with) searches for
/// linkages.
pub struct ParseOptions {
    raw: NonNull<sys::Parse_Options_s>,
}

impl ParseOptions {
    /// link-grammar's default options.
    pub fn new() -> Self { Self::with_verbosity(&verbosity::current()) }

    /// The most linkages a parse keeps. Values past `i32::MAX` keep
    /// `i32::MAX`.
    pub fn set_linkage_limit(&mut self, limit: usize) {
        unsafe {
            sys::parse_options_set_linkage_limit(
                self.as_raw(),
                saturating_c_int(limit),
            )
        };
    }

    /// The most words a linkage may leave unlinked, so ungrammatical
    /// sentences still parse. The default is 0.
    pub fn set_max_null_count(&mut self, count: usize) {
        unsafe {
            sys::parse_options_set_max_null_count(
                self.as_raw(),
                saturating_c_int(count),
            )
        };
    }

    /// Default options at `verbosity`, which also becomes link-grammar's
    /// global level.
    pub(crate) fn with_verbosity(
        verbosity: &MutexGuard<'_, Verbosity>,
    ) -> Self {
        // SAFETY: `parse_options_create` takes no arguments.
        let raw = unsafe { sys::parse_options_create() };
        let options = Self {
            raw: NonNull::new(raw).expect(
                "parse_options_create only fails when malloc does",
            ),
        };
        options.write_verbosity(**verbosity);
        options
    }

    /// Copies the process-wide level into these options, which keep their
    /// own copy for the step times they print.
    pub(crate) fn sync_verbosity(&self) {
        let verbosity = verbosity::current();
        self.write_verbosity(*verbosity);
    }

    /// Sets the options' level and link-grammar's global level.
    fn write_verbosity(&self, verbosity: Verbosity) {
        unsafe {
            sys::parse_options_set_verbosity(
                self.as_raw(),
                verbosity.as_raw(),
            )
        };
    }

    /// The handle the C API takes.
    pub(crate) fn as_raw(&self) -> sys::Parse_Options { self.raw.as_ptr() }
}

impl Default for ParseOptions {
    fn default() -> Self { Self::new() }
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
