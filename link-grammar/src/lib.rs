//! Safe Rust bindings to the [link-grammar] parser.
//!
//! A [`Dictionary`] holds a language's grammar. A [`Sentence`] borrows it,
//! and parsing the sentence finds its [`Linkage`]s, each a set of
//! [`Link`]s between [`Word`]s. A link's [`LinkLabel`] splits into a
//! [`LinkType`] and a [`Subscript`].
//!
//! Each [`Language`] is a marker type, such as [`english::English`], with
//! its own relation enum, such as [`english::Relation`], whose variants
//! the link types name. A type that leaves the language out means English.
//!
//! [link-grammar]: https://github.com/opencog/link-grammar

#[macro_use]
mod macros;

mod c_str_ext;
mod dictionary;
#[cfg(test)]
mod differential_tests;
pub mod english;
mod error;
mod internals_ext;
mod language;
mod link_label;
mod linkage;
mod parse_options;
mod sentence;
mod subscript;
mod verbosity;
mod word;

pub use dictionary::{Dictionary, set_data_dir};
pub use error::{Error, Result};
pub use language::{Language, LinkRelation};
pub use link_label::{LinkLabel, LinkType};
pub use linkage::{Link, Linkage};
pub use sentence::Sentence;
pub use subscript::Subscript;
pub use verbosity::{Verbosity, set_verbosity};
pub use word::Word;
