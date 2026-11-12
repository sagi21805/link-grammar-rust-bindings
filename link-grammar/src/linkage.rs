use std::ffi::CStr;
use std::marker::PhantomData;
use std::ptr::NonNull;

use link_grammar_sys as sys;

use crate::c_str_ext::CStrPtr;

/// One parse of a [`Sentence`](crate::Sentence). Its storage belongs to
/// the sentence, so it borrows the sentence.
///
/// ```no_run
/// # use link_grammar::{Dictionary, ParseOptions, Sentence};
/// # let dictionary = Dictionary::new("en")?;
/// # let options = ParseOptions::new();
/// # let mut sentence = Sentence::new(&dictionary, "Mars is red.")?;
/// sentence.parse(&options)?;
/// let linkage = sentence.linkage(0, &options).expect("a parse");
/// let words: Vec<&str> = linkage.words().collect();
/// for link in linkage.links() {
///     println!("{} -{}- {}", words[link.left], link.label, words[link.right]);
/// }
/// # Ok::<(), link_grammar::Error>(())
/// ```
pub struct Linkage<'s> {
    raw: NonNull<sys::Linkage_s>,
    sentence: PhantomData<&'s ()>,
}

/// A labelled link between two words of a [`Linkage`], given as indices
/// into [`Linkage::words`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Link<'l> {
    pub left: usize,
    pub right: usize,
    pub label: &'l str,
}

impl<'s> Linkage<'s> {
    /// Wraps a handle from `linkage_create`.
    ///
    /// # Safety
    ///
    /// `raw` is a linkage of a sentence that outlives `'s` and isn't
    /// reparsed during it.
    pub(crate) unsafe fn from_raw(raw: NonNull<sys::Linkage_s>) -> Self {
        Self {
            raw,
            sentence: PhantomData,
        }
    }

    /// The words of the linkage, left wall first, each with the
    /// dictionary subscript it matched (`"touched.v-d"`).
    pub fn words(&self) -> impl ExactSizeIterator<Item = &str> {
        // SAFETY: `raw` is a live linkage.
        let count = unsafe { sys::linkage_get_num_words(self.as_raw()) };
        (0..count).map(|index| {
            // SAFETY: `index` is below the word count, and the word lives
            // as long as the linkage.
            unsafe {
                sys::linkage_get_word(self.as_raw(), index).as_utf8()
            }
        })
    }

    /// The links of the linkage.
    pub fn links(&self) -> impl ExactSizeIterator<Item = Link<'_>> {
        // SAFETY: `raw` is a live linkage.
        let count = unsafe { sys::linkage_get_num_links(self.as_raw()) };
        (0..count).map(|index| self.link(index))
    }

    /// The ASCII diagram link-parser prints, wrapped to `width` columns.
    pub fn diagram(&self, width: usize) -> String {
        // SAFETY: `raw` is a live linkage.
        let raw = unsafe {
            sys::linkage_print_diagram(self.as_raw(), true, width)
        };
        // SAFETY: `linkage_print_diagram` returns a NUL-terminated string.
        let diagram = unsafe { CStr::from_ptr(raw) }
            .to_string_lossy()
            .into_owned();
        // SAFETY: `raw` came from `linkage_print_diagram` and is freed
        // once.
        unsafe { sys::linkage_free_diagram(raw) };
        diagram
    }

    /// The link at `index`, which is below the link count.
    fn link(&self, index: usize) -> Link<'_> {
        let raw = self.as_raw();
        // SAFETY: `index` is below the link count, and the label lives as
        // long as the linkage.
        unsafe {
            Link {
                left: sys::linkage_get_link_lword(raw, index),
                right: sys::linkage_get_link_rword(raw, index),
                label: sys::linkage_get_link_label(raw, index).as_utf8(),
            }
        }
    }

    /// The handle the C API takes.
    fn as_raw(&self) -> sys::Linkage { self.raw.as_ptr() }
}

impl Drop for Linkage<'_> {
    fn drop(&mut self) {
        // SAFETY: `raw` came from `linkage_create` and is deleted once.
        unsafe { sys::linkage_delete(self.raw.as_ptr()) };
    }
}
