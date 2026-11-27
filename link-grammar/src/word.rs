use std::fmt;
use std::hash::{Hash, Hasher};

use link_grammar_sys::internals;

use crate::english::English;
use crate::internals_ext::{DisjunctInternals, LinkageInternals};
use crate::{Language, Link, Linkage};

/// A word of a [`Linkage`](crate::Linkage), as link-grammar prints it.
///
/// The printed form is the word, a guess mark in brackets when the
/// dictionary had to guess, and a `.` with the dictionary entry's
/// subscript, as in `lander[?].n`, `Mr..x` and `123[!]`. A word that a
/// parse with nulls left unlinked prints in brackets, as `[the]`.
#[derive(Clone, Copy)]
pub struct Word<'l, L: Language = English> {
    /// The word's position in `linkage`, with the left wall at 0.
    index: usize,
    linkage: &'l Linkage<'l, L>,
}

impl<'l, L: Language> Word<'l, L> {
    /// The word of `linkage` at `index`, which is below the word count.
    pub(crate) fn new(index: usize, linkage: &'l Linkage<'l, L>) -> Self {
        Self { index, linkage }
    }

    /// The word as link-grammar prints it, such as `lander[?].n`.
    pub fn as_str(self) -> &'l str {
        self.linkage.printed_word(self.index)
    }

    /// The links with an end at the word, in the linkage's order.
    pub fn links(self) -> impl Iterator<Item = Link<'l, L>> {
        self.linkage.links().filter(move |link| {
            link.left == self.index || link.right == self.index
        })
    }

    /// The word itself, without its guess mark or subscript, such as
    /// `lander` in `lander[?].n`.
    pub fn text(self) -> &'l str {
        match self.entry() {
            Some(entry) => entry
                .rsplit_once(SUBSCRIPT_MARK)
                .map_or(entry, |(text, _)| text),
            None => self.unlinked_inner().unwrap_or(self.as_str()),
        }
    }

    /// The dictionary entry the word used, such as `dog\x03n`, or `None`
    /// for an unlinked word.
    fn entry(self) -> Option<&'l str> { Some(self.disjunct()?.entry()) }

    /// The disjunct the word used, or `None` for an unlinked word.
    fn disjunct(self) -> Option<&'l internals::Disjunct_struct> {
        self.linkage.internals().chosen_disjuncts()[self.index]
    }

    /// The printed form without its brackets, if the word is unlinked.
    fn unlinked_inner(self) -> Option<&'l str> {
        self.as_str()
            .strip_prefix('[')
            .and_then(|inner| inner.strip_suffix(']'))
            .filter(|inner| !inner.is_empty())
    }
}

impl<L: Language> fmt::Display for Word<'_, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<L: Language> PartialEq<&str> for Word<'_, L> {
    fn eq(&self, other: &&str) -> bool { self.as_str() == *other }
}

/// Words are equal when they print the same at the same position, so a
/// word equals itself in another linkage of the same parse. Within one
/// linkage, the position settles it.
impl<L: Language> PartialEq for Word<'_, L> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
            // The C pointer, not the `&Linkage`, because each call to
            // `Sentence::linkage` builds a new wrapper.
            && (self.linkage.as_raw() == other.linkage.as_raw()
                || self.as_str() == other.as_str())
    }
}

impl<L: Language> Eq for Word<'_, L> {}

impl<L: Language> Hash for Word<'_, L> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.as_str().hash(state);
    }
}

/// The printed form and position, followed by the word's text.
impl<L: Language> fmt::Debug for Word<'_, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Word")
            .field("printed", &self.as_str())
            .field("index", &self.index)
            .field("text", &self.text())
            .finish()
    }
}

/// The mark between a dictionary entry's word and its subscript.
const SUBSCRIPT_MARK: char = internals::SUBSCRIPT_MARK as char;
