//! Safe Rust bindings to the [link-grammar] parser.
//!
//! A [`Dictionary`] holds a language's grammar. A [`Sentence`] borrows it,
//! and parsing the sentence finds its [`Linkage`]s, each a set of
//! [`Link`]s between [`Word`]s. A link's [`LinkLabel`] splits into a
//! [`LinkType`], which names a [`Relation`], and a [`Subscript`].
//!
//! [link-grammar]: https://github.com/opencog/link-grammar

#[macro_use]
mod macros;

mod c_str_ext;
mod dictionary;
#[cfg(test)]
mod differential_tests;
mod error;
mod internals_ext;
mod language;
mod link_label;
mod linkage;
mod parse_options;
mod relation;
mod sentence;
mod subscript;
mod verbosity;
mod word;

pub use dictionary::Dictionary;
pub use error::{Error, Result};
pub use language::Language;
pub use link_label::{LinkLabel, LinkType};
pub use linkage::{Link, Linkage};
pub use relation::Relation;
pub use sentence::Sentence;
pub use subscript::Subscript;
pub use verbosity::{Verbosity, set_verbosity};
pub use word::Word;
