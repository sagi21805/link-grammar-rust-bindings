//! Safe Rust bindings to the [link-grammar] parser.
//!
//! A [`Dictionary`] holds a language's grammar. A [`Sentence`] borrows it,
//! and parsing the sentence with [`ParseOptions`] finds its [`Linkage`]s,
//! each a set of labelled [`Link`]s between words.
//!
//! ```
//! use link_grammar::{Dictionary, ParseOptions, Sentence};
//!
//! let dictionary = Dictionary::new("en")?;
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

mod c_str_ext;
mod dictionary;
mod error;
mod linkage;
mod parse_options;
mod sentence;

pub use dictionary::Dictionary;
pub use error::{Error, Result};
pub use linkage::{Link, Linkage};
pub use parse_options::ParseOptions;
pub use sentence::Sentence;
