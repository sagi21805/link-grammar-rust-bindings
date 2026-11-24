use std::ffi::c_int;
use std::sync::{Mutex, MutexGuard};

use crate::ParseOptions;

/// How much link-grammar prints to stderr. Each level adds to the ones
/// before it.
///
/// ```
/// use link_grammar::Verbosity;
///
/// link_grammar::set_verbosity(Verbosity::Silent);
/// # link_grammar::set_verbosity(Verbosity::Basic);
/// ```
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default,
)]
pub enum Verbosity {
    /// Nothing.
    Silent = 0,
    /// Errors and warnings, and where the dictionary was found.
    #[default]
    Basic = 1,
    /// The time each parse step takes.
    Times = 2,
    /// More informational messages.
    Info = 3,
    /// The dictionary file search and the locale setup.
    Files = 4,
}

/// Sets how much link-grammar prints to stderr.
///
/// The level applies to the whole process, because link-grammar keeps it
/// in a global variable. Every [`Sentence::parse`](crate::Sentence::parse)
/// after this call uses it, whichever [`ParseOptions`] it gets.
pub fn set_verbosity(verbosity: Verbosity) {
    let mut level = current();
    *level = verbosity;
    // `parse_options_set_verbosity` is the only way to write the global,
    // and it needs an options object.
    drop(ParseOptions::with_verbosity(&level));
}

impl Verbosity {
    /// The level the C API takes.
    pub(crate) fn as_raw(self) -> c_int { self as c_int }
}

/// The level [`set_verbosity`] chose. Holding the guard keeps it from
/// changing.
pub(crate) fn current() -> MutexGuard<'static, Verbosity> {
    VERBOSITY.lock().expect("no panic while locked")
}

/// The level link-grammar's global should have. `parse_options_create`
/// resets the global to [`Verbosity::Basic`], so the crate reapplies this
/// after creating options.
static VERBOSITY: Mutex<Verbosity> = Mutex::new(Verbosity::Basic);
