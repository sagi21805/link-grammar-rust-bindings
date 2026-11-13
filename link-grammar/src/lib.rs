//! Safe Rust bindings to the [link-grammar] parser.
//!
//! A [`Dictionary`] holds a language's grammar. A [`Sentence`] borrows it,
//! and parsing the sentence finds its [`Linkage`]s, each a set of
//! [`Link`]s between words. [`ParseOptions`] change how a parse searches.
//! A link's [`LinkLabel`] splits into a [`LinkType`], which names a
//! [`Relation`], and a [`Subscript`]. A [`Word`]'s subscript names its
//! [`PartOfSpeech`].
//!
//! ```
//! use link_grammar::{Dictionary, Language, Sentence};
//!
//! let dictionary = Dictionary::new(Language::English)?;
//! let mut sentence = Sentence::new(&dictionary, "The lander landed.")?;
//! sentence.parse()?;
//! for linkage in sentence.linkages() {
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
mod part_of_speech;
mod relation;
mod sentence;
mod verbosity;
mod word;

pub use dictionary::Dictionary;
pub use error::{Error, Result};
pub use language::Language;
pub use link_label::{LinkLabel, LinkType, Subscript};
pub use linkage::{Diagram, Link, Linkage};
pub use parse_options::ParseOptions;
pub use part_of_speech::PartOfSpeech;
pub use relation::Relation;
pub use sentence::Sentence;
pub use verbosity::{Verbosity, set_verbosity};
pub use word::{Word, WordGuess};
