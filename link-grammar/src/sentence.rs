use std::cell::OnceCell;
use std::ffi::CString;
use std::iter;
use std::marker::PhantomData;
use std::ptr::NonNull;

use link_grammar_sys as sys;

use crate::{Dictionary, Error, Linkage, ParseOptions, Result};

/// A sentence to parse with a [`Dictionary`].
///
/// [`parse`](Self::parse) takes `&mut self`, as reparsing frees the
/// linkages of the previous parse. The borrow checker therefore rejects
/// code that keeps a [`Linkage`] across a reparse.
///
/// ```compile_fail,E0502
/// # use link_grammar::{Dictionary, Language, ParseOptions, Sentence};
/// # let dictionary = Dictionary::new(Language::English)?;
/// # let options = ParseOptions::new();
/// let mut sentence = Sentence::new(&dictionary, "Mars is red.")?;
/// sentence.parse(&options)?;
/// let first = sentence.linkage(0, &options);
/// // Doesn't compile, because the second parse frees `first`.
/// sentence.parse(&options)?;
/// drop(first);
/// # Ok::<(), link_grammar::Error>(())
/// ```
pub struct Sentence<'d> {
    raw: NonNull<sys::Sentence_s>,
    /// One cell per linkage the last parse kept, filled the first time
    /// that linkage is asked for. Each `linkage_create` call allocates
    /// the linkage's word list again and leaks the old one, so it runs
    /// once per linkage.
    linkages: Vec<OnceCell<NonNull<sys::Linkage_s>>>,
    dictionary: PhantomData<&'d Dictionary>,
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
            linkages: Vec::new(),
            dictionary: PhantomData,
        })
    }

    /// Splits the sentence into words and finds its linkages. Returns how
    /// many linkages have no post-processing violations.
    ///
    /// ```
    /// use link_grammar::{Dictionary, Language, ParseOptions, Sentence};
    ///
    /// let dictionary = Dictionary::new(Language::English)?;
    /// let options = ParseOptions::new();
    /// let mut sentence = Sentence::new(&dictionary, "Mars is red.")?;
    /// assert!(sentence.parse(&options)? > 0);
    /// # Ok::<(), link_grammar::Error>(())
    /// ```
    pub fn parse(&mut self, options: &ParseOptions) -> Result<usize> {
        self.linkages.clear();
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
        self.linkages =
            iter::repeat_with(OnceCell::new).take(kept).collect();
        Ok(valid)
    }

    /// The linkage at `index`, or `None` past the last one the parse
    /// kept. `options` only matter the first time a linkage is asked for
    /// after a parse.
    ///
    /// ```
    /// use link_grammar::{Dictionary, Language, ParseOptions, Sentence};
    ///
    /// let dictionary = Dictionary::new(Language::English)?;
    /// let options = ParseOptions::new();
    /// let mut sentence = Sentence::new(&dictionary, "Mars is red.")?;
    /// sentence.parse(&options)?;
    /// assert!(sentence.linkage(0, &options).is_some());
    /// assert!(sentence.linkage(usize::MAX, &options).is_none());
    /// # Ok::<(), link_grammar::Error>(())
    /// ```
    pub fn linkage(
        &self,
        index: usize,
        options: &ParseOptions,
    ) -> Option<Linkage<'_>> {
        let raw = *self.linkages.get(index)?.get_or_init(|| {
            // SAFETY: Both handles are live, and `index` is below the
            // number of linkages the parse kept.
            let raw = unsafe {
                sys::linkage_create(index, self.as_raw(), options.as_raw())
            };
            NonNull::new(raw).expect(
                "linkage_create succeeds below the number of kept \
                 linkages",
            )
        });
        // SAFETY: `raw` is a linkage of this sentence, and the returned
        // `Linkage` borrows `self`, so no reparse happens while it lives.
        Some(unsafe { Linkage::from_raw(raw) })
    }

    /// Every linkage the parse kept, best first.
    pub fn linkages<'s>(
        &'s self,
        options: &'s ParseOptions,
    ) -> impl Iterator<Item = Linkage<'s>> {
        (0..).map_while(|index| self.linkage(index, options))
    }

    /// How many words each of the best linkages leaves unlinked.
    pub fn null_count(&self) -> usize {
        // SAFETY: `raw` is a live sentence.
        let count = unsafe { sys::sentence_null_count(self.as_raw()) };
        usize::try_from(count).expect("a null count is never negative")
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
