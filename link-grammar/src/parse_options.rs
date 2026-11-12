use std::ptr::NonNull;

use link_grammar_sys as sys;

/// Settings that control how [`Sentence::parse`](crate::Sentence::parse)
/// searches for linkages.
///
/// ```
/// use link_grammar::ParseOptions;
///
/// let mut options = ParseOptions::new();
/// options.set_verbosity(0);
/// options.set_max_null_count(3);
/// ```
pub struct ParseOptions {
    raw: NonNull<sys::Parse_Options_s>,
}

impl ParseOptions {
    /// link-grammar's default options.
    pub fn new() -> Self {
        // SAFETY: `parse_options_create` takes no arguments.
        let raw = unsafe { sys::parse_options_create() };
        let raw = NonNull::new(raw)
            .expect("parse_options_create only fails when malloc does");
        Self { raw }
    }

    /// How much link-grammar prints to stderr. 0 is silent and 1 is the
    /// default.
    pub fn set_verbosity(&mut self, verbosity: i32) {
        // SAFETY: `raw` is a live options object.
        unsafe {
            sys::parse_options_set_verbosity(self.as_raw(), verbosity)
        };
    }

    /// The most linkages a parse keeps.
    pub fn set_linkage_limit(&mut self, limit: i32) {
        // SAFETY: `raw` is a live options object.
        unsafe {
            sys::parse_options_set_linkage_limit(self.as_raw(), limit)
        };
    }

    /// The most words a linkage may leave unlinked, so ungrammatical
    /// sentences still parse. The default is 0.
    pub fn set_max_null_count(&mut self, count: i32) {
        // SAFETY: `raw` is a live options object.
        unsafe {
            sys::parse_options_set_max_null_count(self.as_raw(), count)
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
