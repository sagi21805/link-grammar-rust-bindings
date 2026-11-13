use std::fmt;

use crate::{Language, PartOfSpeech};

/// A word of a [`Linkage`](crate::Linkage), as link-grammar prints it.
///
/// The printed form is the word, a [`WordGuess`] mark in brackets when the
/// dictionary had to guess, and a `.` with the dictionary entry's
/// subscript: `lander[?].n`, `Mr..x`, `123[!]`. A word that a parse with
/// nulls left unlinked prints in brackets, as `[the]`. A word borrows the
/// linkage's string, and each method scans only the word's few bytes.
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
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Word<'l> {
    printed: &'l str,
    /// The language of the dictionary that parsed the word.
    language: Option<Language>,
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
    /// Wraps a word that a dictionary of `language` printed.
    pub(crate) fn new(
        printed: &'l str,
        language: Option<Language>,
    ) -> Self {
        Self { printed, language }
    }

    /// The word itself, without its guess mark or subscript, such as
    /// `lander` in `lander[?].n`.
    pub fn text(self) -> &'l str { self.parts().text }

    /// The subscript of the dictionary entry the word matched, such as `n`
    /// in `lander[?].n`.
    pub fn subscript(self) -> Option<&'l str> { self.parts().subscript }

    /// Why the dictionary guessed the word, if it did.
    pub fn guess(self) -> Option<WordGuess> { self.parts().guess }

    /// The word class the [`subscript`](Self::subscript) names. It is
    /// `None` for a word without a subscript, for a subscript that names
    /// a narrower group, and for every language but English.
    ///
    /// ```
    /// use link_grammar::{Dictionary, Language, PartOfSpeech, Sentence};
    ///
    /// let dictionary = Dictionary::new(Language::English)?;
    /// let mut sentence = Sentence::new(&dictionary, "The lander landed.")?;
    /// sentence.parse()?;
    /// let linkage = sentence.linkage(0).expect("a parse");
    /// let classes: Vec<_> =
    ///     linkage.words().map(|word| word.part_of_speech()).collect();
    /// assert_eq!(classes[3], Some(PartOfSpeech::Verb));
    /// # Ok::<(), link_grammar::Error>(())
    /// ```
    pub fn part_of_speech(self) -> Option<PartOfSpeech> {
        match self.language {
            Some(Language::English) => {
                PartOfSpeech::from_english_subscript(self.subscript()?)
            }
            _ => None,
        }
    }

    /// Whether the linkage leaves the word unlinked, which only a parse
    /// with [`set_max_null_count`] above 0 does.
    ///
    /// [`set_max_null_count`]: crate::ParseOptions::set_max_null_count
    pub fn is_unlinked(self) -> bool { self.unlinked_inner().is_some() }

    /// The printed form without its brackets, if the word is unlinked.
    fn unlinked_inner(self) -> Option<&'l str> {
        self.printed
            .strip_prefix('[')
            .and_then(|inner| inner.strip_suffix(']'))
            .filter(|inner| !inner.is_empty())
    }

    /// The printed form split into the word, the guess and the subscript.
    fn parts(self) -> WordParts<'l> {
        let printed = self.unlinked_inner().unwrap_or(self.printed);
        WordParts::with_guess(printed)
            .unwrap_or_else(|| WordParts::without_guess(printed))
    }
}

borrowed_str!(Word.printed);

/// The printed form, followed by what the crate reads from it.
///
/// ```
/// use link_grammar::{Dictionary, Language, Sentence};
///
/// let dictionary = Dictionary::new(Language::English)?;
/// let mut sentence = Sentence::new(&dictionary, "The lander landed.")?;
/// sentence.parse()?;
/// let linkage = sentence.linkage(0).expect("a parse");
/// let lander = linkage.words().nth(2).expect("a third word");
/// assert_eq!(
///     format!("{lander:?}"),
///     r#"Word { printed: "lander[?].n", parts: WordParts { text: "lander", guess: Some(Unknown), subscript: Some("n") }, part_of_speech: Some(Noun), unlinked: false }"#,
/// );
/// # Ok::<(), link_grammar::Error>(())
/// ```
impl fmt::Debug for Word<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Word")
            .field("printed", &self.printed)
            .field("parts", &self.parts())
            .field("part_of_speech", &self.part_of_speech())
            .field("unlinked", &self.is_unlinked())
            .finish()
    }
}

/// The pieces of a printed word.
#[derive(Debug)]
struct WordParts<'l> {
    text: &'l str,
    guess: Option<WordGuess>,
    subscript: Option<&'l str>,
}

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
