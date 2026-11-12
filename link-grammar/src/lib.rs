//! Safe Rust bindings to the [link-grammar] parser.
//!
//! A [`Dictionary`] holds a language's grammar. A [`Sentence`] borrows it,
//! and parsing the sentence with [`ParseOptions`] finds its [`Linkage`]s,
//! each a set of [`Link`]s between words. A link's [`LinkLabel`] splits
//! into a [`LinkType`] and a [`Subscript`].
//!
//! ```
//! use link_grammar::{Dictionary, Language, ParseOptions, Sentence};
//!
//! let dictionary = Dictionary::new(Language::English)?;
//! let options = ParseOptions::new();
//! let mut sentence = Sentence::new(&dictionary, "The lander landed.")?;
//! sentence.parse(&options)?;
//! for linkage in sentence.linkages(&options) {
//!     println!("{}", linkage.diagram(80));
//! }
//! # Ok::<(), link_grammar::Error>(())
//! ```
//!
//! [link-grammar]: https://github.com/opencog/link-grammar

#[macro_use]
mod macros;

mod c_str_ext;
mod dictionary;
mod error;
mod language;
mod link_label;
mod linkage;
mod parse_options;
mod sentence;
mod verbosity;
mod word;

pub use dictionary::Dictionary;
pub use error::{Error, Result};
pub use language::Language;
pub use link_label::{LinkLabel, LinkType, Subscript};
pub use linkage::{Link, Linkage};
pub use parse_options::ParseOptions;
pub use sentence::Sentence;
pub use verbosity::{Verbosity, set_verbosity};
pub use word::{Word, WordGuess};
