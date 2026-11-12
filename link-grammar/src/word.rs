/// A word of a [`Linkage`](crate::Linkage), as link-grammar prints it.
///
/// The printed form is the word, a [`WordGuess`] mark in brackets when the
/// dictionary had to guess, and a `.` with the dictionary entry's
/// subscript: `lander[?].n`, `Mr..x`, `123[!]`. A word that a parse with
/// nulls left unlinked prints in brackets, as `[the]`. A word borrows the
/// linkage's string, and each method scans only the word's few bytes.
///
/// ```
/// use link_grammar::{Dictionary, Language, ParseOptions, Sentence, WordGuess};
///
/// let dictionary = Dictionary::new(Language::English)?;
/// let options = ParseOptions::new();
/// let mut sentence = Sentence::new(&dictionary, "Mr. Smith bought 123 widgets.")?;
/// sentence.parse(&options)?;
/// let linkage = sentence.linkage(0, &options).expect("a parse");
/// let words: Vec<_> = linkage.words().collect();
/// assert_eq!(words[1], "Mr..x");
/// assert_eq!(words[1].text(), "Mr.");
/// assert_eq!(words[1].subscript(), Some("x"));
/// assert_eq!(words[4].text(), "123");
/// assert_eq!(words[4].guess(), Some(WordGuess::Regex));
/// # Ok::<(), link_grammar::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Word<'l>(&'l str);

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
    /// Wraps a word that link-grammar printed.
    pub(crate) fn new(word: &'l str) -> Self { Self(word) }

    /// The word itself, without its guess mark or subscript, such as
    /// `lander` in `lander[?].n`.
    pub fn text(self) -> &'l str { self.parts().text }

    /// The subscript of the dictionary entry the word matched, such as `n`
    /// in `lander[?].n`.
    pub fn subscript(self) -> Option<&'l str> { self.parts().subscript }

    /// Why the dictionary guessed the word, if it did.
    pub fn guess(self) -> Option<WordGuess> { self.parts().guess }

    /// Whether the linkage leaves the word unlinked, which only a parse
    /// with [`set_max_null_count`] above 0 does.
    ///
    /// [`set_max_null_count`]: crate::ParseOptions::set_max_null_count
    pub fn is_unlinked(self) -> bool { self.unlinked_inner().is_some() }

    /// The printed form without its brackets, if the word is unlinked.
    fn unlinked_inner(self) -> Option<&'l str> {
        self.0
            .strip_prefix('[')
            .and_then(|inner| inner.strip_suffix(']'))
            .filter(|inner| !inner.is_empty())
    }

    /// The printed form split into the word, the guess and the subscript.
    fn parts(self) -> WordParts<'l> {
        let printed = self.unlinked_inner().unwrap_or(self.0);
        WordParts::with_guess(printed)
            .unwrap_or_else(|| WordParts::without_guess(printed))
    }
}

borrowed_str!(Word);

/// The pieces of a printed word.
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
