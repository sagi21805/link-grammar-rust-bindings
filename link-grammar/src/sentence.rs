use std::ffi::{CString, c_int};
use std::marker::PhantomData;
use std::ptr::NonNull;

use link_grammar_sys as sys;

use crate::english::English;
use crate::parse_options::ParseOptions;
use crate::{Dictionary, Error, Language, Linkage, Result};

/// A sentence to parse with a [`Dictionary`].
///
/// [`parse`](Self::parse) takes `&mut self`, as reparsing frees the
/// linkages of the previous parse. The borrow checker therefore rejects
/// code that keeps a [`Linkage`] across a reparse.
///
/// ```compile_fail,E0502
/// # use link_grammar::english::English;
/// # use link_grammar::{Dictionary, Sentence};
/// # let dictionary = Dictionary::new(English)?;
/// let mut sentence = Sentence::new(&dictionary, "Mars is red.")?;
/// sentence.parse()?;
/// let first = sentence.linkage(0);
/// // Doesn't compile, because the second parse frees `first`.
/// sentence.parse()?;
/// drop(first);
/// # Ok::<(), link_grammar::Error>(())
/// ```
pub struct Sentence<'d, L: Language = English> {
    raw: NonNull<sys::Sentence_s>,
    /// The linkages the last parse kept, best first, and empty before one
    /// succeeds.
    linkages: Vec<NonNull<sys::Linkage_s>>,
    /// The C sentence points into the dictionary.
    dictionary: PhantomData<&'d Dictionary<L>>,
}

impl<'d, L: Language> Sentence<'d, L> {
    /// Copies `text` into a sentence that `dictionary` will parse.
    pub fn new(dictionary: &'d Dictionary<L>, text: &str) -> Result<Self> {
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

    /// Splits the sentence into words and finds its linkages with
    /// link-grammar's default options, best first, so
    /// [`linkage(0)`](Self::linkage) is the one link-parser shows first.
    /// When no linkage connects every word, the linkages leave the fewest
    /// words unlinked that they can. Returns how many linkages have no
    /// post-processing violations.
    pub fn parse(&mut self) -> Result<usize> {
        self.linkages.clear();
        let mut options = ParseOptions::new();
        let mut valid = self.run_parse(&options)?;
        if valid == 0 {
            options.set_min_null_count(1);
            options.set_max_null_count(self.length());
            valid = self.run_parse(&options)?;
        }
        // `linkage_create` rewrites a linkage's words and links in place,
        // so each linkage is created once, here.
        self.linkages = (0..self.kept())
            .map(|index| self.create_linkage(index, &options))
            .collect();
        Ok(valid)
    }

    /// The linkage at `index`, or `None` past the last one the parse
    /// kept or before the first parse.
    pub fn linkage(&self, index: usize) -> Option<Linkage<'_, L>> {
        let raw = *self.linkages.get(index)?;
        // SAFETY: `raw` is a linkage of this sentence, and the returned
        // `Linkage` borrows `self`, so no reparse happens while it lives.
        Some(unsafe { Linkage::from_raw(raw, self) })
    }

    /// Every linkage the parse kept, best first.
    pub fn linkages(
        &self,
    ) -> impl ExactSizeIterator<Item = Linkage<'_, L>> {
        (0..self.linkages.len()).map(|index| {
            self.linkage(index)
                .expect("index is below the linkage count")
        })
    }

    /// How many words each of the best linkages leaves unlinked.
    pub fn null_count(&self) -> usize {
        self.count(sys::sentence_null_count)
    }

    /// Runs `sentence_parse` and returns how many linkages are valid.
    fn run_parse(&mut self, options: &ParseOptions) -> Result<usize> {
        // SAFETY: Both handles are live, and `&mut self` guarantees no
        // `Linkage` into the storage this frees is alive.
        let status = unsafe {
            sys::sentence_parse(self.as_raw(), options.as_raw())
        };
        match status {
            -1 => Err(Error::Split),
            -2 => Err(Error::TooLong),
            -3 => Err(Error::GenerationMode),
            count => Ok(usize::try_from(count)
                .expect("sentence_parse returns -1, -2, -3 or a count")),
        }
    }

    /// `linkage_create` for `index`, which is below [`kept`](Self::kept).
    fn create_linkage(
        &self,
        index: usize,
        options: &ParseOptions,
    ) -> NonNull<sys::Linkage_s> {
        // SAFETY: Both handles are live, and `index` is below the number
        // of linkages the parse kept.
        let raw = unsafe {
            sys::linkage_create(index, self.as_raw(), options.as_raw())
        };
        NonNull::new(raw).expect(
            "linkage_create succeeds below the number of kept linkages",
        )
    }

    /// How many words link-grammar split the sentence into.
    fn length(&self) -> usize { self.count(sys::sentence_length) }

    /// How many linkages the last parse kept.
    fn kept(&self) -> usize {
        self.count(sys::sentence_num_linkages_post_processed)
    }

    /// A count that `get` reads from the sentence.
    fn count(
        &self,
        get: unsafe extern "C" fn(sys::Sentence) -> c_int,
    ) -> usize {
        // SAFETY: `get` only reads the sentence.
        let count = unsafe { get(self.as_raw()) };
        usize::try_from(count)
            .expect("link-grammar's counts are never negative")
    }

    /// The handle the C API takes.
    fn as_raw(&self) -> sys::Sentence { self.raw.as_ptr() }
}

impl<L: Language> Drop for Sentence<'_, L> {
    fn drop(&mut self) {
        // SAFETY: `raw` came from `sentence_create`, and every linkage
        // borrowing it is already gone.
        unsafe { sys::sentence_delete(self.raw.as_ptr()) };
    }
}
