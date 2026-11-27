use std::fmt;
use std::hash::{Hash, Hasher};

#[cfg(feature = "bundled")]
use link_grammar_sys::internals;

#[cfg(feature = "bundled")]
use crate::internals_ext::{
    DisjunctInternals, LinkageInternals, WordNode,
};
use crate::{Language, Link, Linkage, PartOfSpeech, Relation};

/// A word of a [`Linkage`](crate::Linkage), as link-grammar prints it.
///
/// The printed form is the word, a [`WordGuess`] mark in brackets when the
/// dictionary had to guess, and a `.` with the dictionary entry's
/// subscript, as in `lander[?].n`, `Mr..x` and `123[!]`. A word that a
/// parse with nulls left unlinked prints in brackets, as `[the]`.
///
/// ```
/// use link_grammar::{Dictionary, Language, Sentence, WordGuess};
///
/// let dictionary = Dictionary::new(Language::English)?;
/// let mut sentence = Sentence::new(&dictionary, "Mr. Smith bought 123 widgets.")?;
/// sentence.parse()?;
/// let linkage = sentence.linkage(0).expect("a parse");
/// let words: Vec<_> = linkage.words().collect();
/// assert_eq!(words[1], "Mr..x");
/// assert_eq!(words[1].text(), "Mr.");
/// assert_eq!(words[1].subscript(), Some("x"));
/// assert_eq!(words[4].text(), "123");
/// assert_eq!(words[4].guess(), Some(WordGuess::Regex));
/// # Ok::<(), link_grammar::Error>(())
/// ```
#[derive(Clone, Copy)]
pub struct Word<'l> {
    /// The word's position in `linkage`, with the left wall at 0.
    index: usize,
    linkage: &'l Linkage<'l>,
}

/// Why link-grammar guessed a [`Word`] instead of finding it in the
/// dictionary. Each variant is the mark it prints in brackets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum WordGuess {
    /// `[!]`, a match for one of the dictionary's regular expressions,
    /// such as a number.
    Regex,
    /// `[~]`, a spelling correction.
    Spelling,
    /// `[&]`, a part of a run-on word that link-grammar split.
    RunOn,
    /// `[?]`, a word the dictionary doesn't have, whose part of speech
    /// link-grammar guessed from the sentence.
    Unknown,
}

impl<'l> Word<'l> {
    /// The word of `linkage` at `index`, which is below the word count.
    pub(crate) fn new(index: usize, linkage: &'l Linkage<'l>) -> Self {
        Self { index, linkage }
    }

    /// The word as link-grammar prints it, such as `lander[?].n`.
    pub fn as_str(self) -> &'l str {
        self.linkage.printed_word(self.index)
    }

    /// The links with an end at the word, in the linkage's order.
    pub fn links(self) -> impl Iterator<Item = Link<'l>> {
        self.linkage.links().filter(move |link| {
            link.left == self.index || link.right == self.index
        })
    }

    /// The first link at the word whose label names `relation`. Use
    /// [`links`](Self::links) to find every such link.
    ///
    /// ```
    /// use link_grammar::{Dictionary, Language, Relation, Sentence};
    ///
    /// let dictionary = Dictionary::new(Language::English)?;
    /// let mut sentence =
    ///     Sentence::new(&dictionary, "The rover drilled the rock slowly.")?;
    /// sentence.parse()?;
    /// let linkage = sentence.linkage(0).expect("a parse");
    /// let object = linkage.link(Relation::Object).expect("an object link");
    /// let verb = object.left_word();
    /// assert_eq!(verb.text(), "drilled");
    /// let manner =
    ///     verb.link(Relation::VerbModifier).expect("a verb modifier link");
    /// assert_eq!(manner.right_word().text(), "slowly");
    /// # Ok::<(), link_grammar::Error>(())
    /// ```
    pub fn link(self, relation: Relation) -> Option<Link<'l>> {
        self.links().find(|link| link.relation() == Some(relation))
    }

    /// The word itself, without its guess mark or subscript, such as
    /// `lander` in `lander[?].n`.
    pub fn text(self) -> &'l str {
        #[cfg(feature = "bundled")]
        return match self.entry() {
            Some(entry) => entry
                .rsplit_once(SUBSCRIPT_MARK)
                .map_or(entry, |(text, _)| text),
            None => self.unlinked_inner().unwrap_or(self.as_str()),
        };
        #[cfg(not(feature = "bundled"))]
        return self.parts().text;
    }

    /// The subscript of the dictionary entry the word matched, such as `n`
    /// in `lander[?].n`.
    pub fn subscript(self) -> Option<&'l str> {
        #[cfg(feature = "bundled")]
        return self
            .entry()?
            .rsplit_once(SUBSCRIPT_MARK)
            .map(|(_, subscript)| subscript)
            .filter(|&subscript| subscript != IDIOM_SUBSCRIPT);
        #[cfg(not(feature = "bundled"))]
        return self.parts().subscript;
    }

    /// Why the dictionary guessed the word, if it did.
    pub fn guess(self) -> Option<WordGuess> {
        #[cfg(feature = "bundled")]
        return self.disjunct().and_then(|_| {
            self.linkage.internals().word_nodes()[self.index].guess()
        });
        #[cfg(not(feature = "bundled"))]
        return self.parts().guess;
    }

    /// The word class the [`subscript`](Self::subscript) names. It is
    /// `None` for a word without a subscript, for a subscript that names
    /// a narrower group, and for every language but English.
    pub fn part_of_speech(self) -> Option<PartOfSpeech> {
        match self.linkage.language() {
            Some(Language::English) => {
                PartOfSpeech::from_english_subscript(self.subscript()?)
            }
            _ => None,
        }
    }

    /// Whether the linkage leaves the word unlinked, which happens only
    /// when no linkage connects every word.
    pub fn is_unlinked(self) -> bool {
        #[cfg(feature = "bundled")]
        return self.disjunct().is_none();
        #[cfg(not(feature = "bundled"))]
        return self.unlinked_inner().is_some();
    }

    /// The dictionary entry the word used, such as `dog\x03n`, or `None`
    /// for an unlinked word.
    #[cfg(feature = "bundled")]
    fn entry(self) -> Option<&'l str> { Some(self.disjunct()?.entry()) }

    /// The disjunct the word used, or `None` for an unlinked word.
    #[cfg(feature = "bundled")]
    fn disjunct(self) -> Option<&'l internals::Disjunct_struct> {
        self.linkage.internals().chosen_disjuncts()[self.index]
    }

    /// The printed form without its brackets, if the word is unlinked.
    pub(crate) fn unlinked_inner(self) -> Option<&'l str> {
        self.as_str()
            .strip_prefix('[')
            .and_then(|inner| inner.strip_suffix(']'))
            .filter(|inner| !inner.is_empty())
    }

    /// The printed form split into the word, the guess and the subscript.
    #[cfg(not(feature = "bundled"))]
    pub(crate) fn parts(self) -> WordParts<'l> {
        let printed = self.unlinked_inner().unwrap_or(self.as_str());
        WordParts::with_guess(printed)
            .unwrap_or_else(|| WordParts::without_guess(printed))
    }
}

impl fmt::Display for Word<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialEq<&str> for Word<'_> {
    fn eq(&self, other: &&str) -> bool { self.as_str() == *other }
}

/// Words are equal when they print the same at the same position, so a
/// word equals itself in another linkage of the same parse.
impl PartialEq for Word<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.as_str() == other.as_str()
    }
}

impl Eq for Word<'_> {}

impl Hash for Word<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.as_str().hash(state);
    }
}

/// The printed form and position, followed by what the crate reads about
/// the word.
impl fmt::Debug for Word<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Word")
            .field("printed", &self.as_str())
            .field("index", &self.index)
            .field("text", &self.text())
            .field("guess", &self.guess())
            .field("subscript", &self.subscript())
            .field("part_of_speech", &self.part_of_speech())
            .field("unlinked", &self.is_unlinked())
            .finish()
    }
}

/// The mark between a dictionary entry's word and its subscript.
#[cfg(feature = "bundled")]
const SUBSCRIPT_MARK: char = internals::SUBSCRIPT_MARK as char;

/// The subscript link-grammar gives idiom words, which it doesn't print.
#[cfg(feature = "bundled")]
const IDIOM_SUBSCRIPT: &str = "_I";

/// The pieces of a printed word.
#[cfg(not(feature = "bundled"))]
#[derive(Debug, PartialEq)]
pub(crate) struct WordParts<'l> {
    pub(crate) text: &'l str,
    pub(crate) guess: Option<WordGuess>,
    pub(crate) subscript: Option<&'l str>,
}

#[cfg(not(feature = "bundled"))]
impl<'l> WordParts<'l> {
    /// `text[mark…]` or `text[mark…].subscript`. The brackets may hold a
    /// regex name after the mark.
    fn with_guess(printed: &'l str) -> Option<Self> {
        let open = printed.rfind('[').filter(|&open| open > 0)?;
        let (text, bracketed) = printed.split_at(open);
        let close = bracketed.find(']')?;
        let guess = WordGuess::from_mark(bracketed[1..].chars().next()?)?;
        let subscript = match &bracketed[close + 1..] {
            "" => None,
            rest => Some(rest.strip_prefix('.')?),
        };
        Some(Self {
            text,
            guess: Some(guess),
            subscript,
        })
    }

    /// `text` or `text.subscript`, where the subscript starts at the last
    /// `.` unless that dot ends the word or comes before a digit or a
    /// non-ASCII byte. That is the rule `patch_subscript` in link-grammar
    /// applies to dictionary entries.
    fn without_guess(printed: &'l str) -> Self {
        let subscript_dot = printed.rfind('.').filter(|&dot| {
            printed[dot + 1..].bytes().next().is_some_and(|next| {
                next.is_ascii() && !next.is_ascii_digit()
            })
        });
        match subscript_dot {
            Some(dot) => Self {
                text: &printed[..dot],
                guess: None,
                subscript: Some(&printed[dot + 1..]),
            },
            None => Self {
                text: printed,
                guess: None,
                subscript: None,
            },
        }
    }
}

#[cfg(not(feature = "bundled"))]
impl WordGuess {
    /// The guess that prints `mark`, one of `!`, `~`, `&` and `?`.
    fn from_mark(mark: char) -> Option<Self> {
        match mark {
            '!' => Some(Self::Regex),
            '~' => Some(Self::Spelling),
            '&' => Some(Self::RunOn),
            '?' => Some(Self::Unknown),
            _ => None,
        }
    }
}
