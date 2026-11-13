use std::cell::OnceCell;
use std::ffi::CString;
use std::iter;
use std::ops::Deref;
use std::ptr::NonNull;

use link_grammar_sys as sys;

use crate::{Dictionary, Error, Linkage, ParseOptions, Result};

/// A sentence to parse with a [`Dictionary`]. It keeps the options of
/// its last parse, which link-grammar reads again when it creates each
/// [`Linkage`].
///
/// [`parse`](Self::parse) takes `&mut self`, as reparsing frees the
/// linkages of the previous parse. The borrow checker therefore rejects
/// code that keeps a [`Linkage`] across a reparse.
///
/// ```compile_fail,E0502
/// # use link_grammar::{Dictionary, Language, Sentence};
/// # let dictionary = Dictionary::new(Language::English)?;
/// let mut sentence = Sentence::new(&dictionary, "Mars is red.")?;
/// sentence.parse()?;
/// let first = sentence.linkage(0);
/// // Doesn't compile, because the second parse frees `first`.
/// sentence.parse()?;
/// drop(first);
/// # Ok::<(), link_grammar::Error>(())
/// ```
pub struct Sentence<'d> {
    raw: NonNull<sys::Sentence_s>,
    /// `None` until a parse succeeds.
    parsed: Option<Parsed<'d>>,
    dictionary: &'d Dictionary,
}

impl<'d> Sentence<'d> {
    /// Copies `text` into a sentence that `dictionary` will parse.
    pub fn new(dictionary: &'d Dictionary, text: &str) -> Result<Self> {
        let c_text = CString::new(text)?;
        // SAFETY: `c_text` is NUL-terminated and link-grammar copies it.
        // The `'d` borrow keeps `dictionary` alive while the sentence
        // points to it.
        let raw = unsafe {
            sys::sentence_create(c_text.as_ptr(), dictionary.as_raw())
        };
        let raw = NonNull::new(raw)
            .expect("sentence_create only fails when malloc does");
        Ok(Self {
            raw,
            parsed: None,
            dictionary,
        })
    }

    /// Splits the sentence into words and finds its linkages with
    /// link-grammar's default options. Returns how many linkages have no
    /// post-processing violations.
    ///
    /// ```
    /// use link_grammar::{Dictionary, Language, Sentence};
    ///
    /// let dictionary = Dictionary::new(Language::English)?;
    /// let mut sentence = Sentence::new(&dictionary, "Mars is red.")?;
    /// assert!(sentence.parse()? > 0);
    /// # Ok::<(), link_grammar::Error>(())
    /// ```
    pub fn parse(&mut self) -> Result<usize> {
        self.parse_keeping(SentenceOptions::Default(ParseOptions::new()))
    }

    /// [`parse`](Self::parse) with `options` instead of the defaults.
    /// The sentence borrows `options` for as long as it lives, so one set
    /// of options can parse many sentences.
    ///
    /// ```compile_fail,E0505
    /// use link_grammar::{Dictionary, Language, ParseOptions, Sentence};
    ///
    /// let dictionary = Dictionary::new(Language::English)?;
    /// let mut options = ParseOptions::new();
    /// options.set_max_null_count(1);
    /// let mut sentence = Sentence::new(
    ///     &dictionary,
    ///     "The the lander touched down on Mars.",
    /// )?;
    /// sentence.parse_with(&options)?;
    /// assert_eq!(sentence.null_count(), 1);
    ///
    /// // Doesn't compile, because the sentence still borrows `options`.
    /// drop(options);
    /// sentence.linkage(0);
    /// # Ok::<(), link_grammar::Error>(())
    /// ```
    pub fn parse_with(
        &mut self,
        options: &'d ParseOptions,
    ) -> Result<usize> {
        self.parse_keeping(SentenceOptions::Borrowed(options))
    }

    /// The linkage at `index`, or `None` past the last one the parse
    /// kept or before the first parse.
    ///
    /// ```
    /// use link_grammar::{Dictionary, Language, Sentence};
    ///
    /// let dictionary = Dictionary::new(Language::English)?;
    /// let mut sentence = Sentence::new(&dictionary, "Mars is red.")?;
    /// assert!(sentence.linkage(0).is_none());
    /// sentence.parse()?;
    /// assert!(sentence.linkage(0).is_some());
    /// assert!(sentence.linkage(usize::MAX).is_none());
    /// # Ok::<(), link_grammar::Error>(())
    /// ```
    pub fn linkage(&self, index: usize) -> Option<Linkage<'_>> {
        let parsed = self.parsed.as_ref()?;
        let raw = *parsed.linkages.get(index)?.get_or_init(|| {
            // SAFETY: Both handles are live, and `index` is below the
            // number of linkages the parse kept.
            let raw = unsafe {
                sys::linkage_create(
                    index,
                    self.as_raw(),
                    parsed.options.as_raw(),
                )
            };
            NonNull::new(raw).expect(
                "linkage_create succeeds below the number of kept \
                 linkages",
            )
        });
        // SAFETY: `raw` is a linkage of this sentence, and the returned
        // `Linkage` borrows `self`, so no reparse happens while it lives.
        Some(unsafe { Linkage::from_raw(raw, self.dictionary.language()) })
    }

    /// Every linkage the parse kept, best first.
    pub fn linkages(&self) -> impl Iterator<Item = Linkage<'_>> {
        (0..).map_while(|index| self.linkage(index))
    }

    /// How many words each of the best linkages leaves unlinked.
    pub fn null_count(&self) -> usize {
        // SAFETY: `raw` is a live sentence.
        let count = unsafe { sys::sentence_null_count(self.as_raw()) };
        usize::try_from(count).expect("a null count is never negative")
    }

    /// Parses with `options` and keeps them for `linkage_create`.
    fn parse_keeping(
        &mut self,
        options: SentenceOptions<'d>,
    ) -> Result<usize> {
        self.parsed = None;
        options.sync_verbosity();
        // SAFETY: Both handles are live, and `&mut self` guarantees no
        // `Linkage` into the storage this frees is alive.
        let status = unsafe {
            sys::sentence_parse(self.as_raw(), options.as_raw())
        };
        let valid = match status {
            -1 => return Err(Error::Split),
            -2 => return Err(Error::TooLong),
            -3 => return Err(Error::GenerationMode),
            count => usize::try_from(count)
                .expect("sentence_parse returns -1, -2, -3 or a count"),
        };
        // SAFETY: `raw` is a live sentence.
        let kept = unsafe {
            sys::sentence_num_linkages_post_processed(self.as_raw())
        };
        let kept = usize::try_from(kept)
            .expect("a linkage count is never negative");
        self.parsed = Some(Parsed {
            options,
            linkages: iter::repeat_with(OnceCell::new)
                .take(kept)
                .collect(),
        });
        Ok(valid)
    }

    /// The handle the C API takes.
    fn as_raw(&self) -> sys::Sentence { self.raw.as_ptr() }
}

impl Drop for Sentence<'_> {
    fn drop(&mut self) {
        // SAFETY: `raw` came from `sentence_create`, and every linkage
        // borrowing it is already gone.
        unsafe { sys::sentence_delete(self.raw.as_ptr()) };
    }
}

/// What a successful parse leaves for `linkage_create`.
struct Parsed<'o> {
    options: SentenceOptions<'o>,
    /// One cell per linkage the parse kept, filled the first time that
    /// linkage is asked for. Each `linkage_create` call allocates the
    /// linkage's word list again and leaks the old one, so it runs once
    /// per linkage.
    linkages: Vec<OnceCell<NonNull<sys::Linkage_s>>>,
}

/// The options a sentence parsed with.
enum SentenceOptions<'o> {
    /// Made by [`Sentence::parse`].
    Default(ParseOptions),
    /// Lent to [`Sentence::parse_with`].
    Borrowed(&'o ParseOptions),
}

impl Deref for SentenceOptions<'_> {
    type Target = ParseOptions;

    fn deref(&self) -> &ParseOptions {
        match self {
            Self::Default(options) => options,
            Self::Borrowed(options) => options,
        }
    }
}
