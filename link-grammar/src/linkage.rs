use std::fmt;
use std::ptr::NonNull;

use link_grammar_sys as sys;

use crate::c_str_ext::CStrPtr;
use crate::internals_ext::{LinkInternals, LinkageInternals};
use crate::{
    Dictionary, Language, LinkLabel, Relation, Sentence, Subscript, Word,
};

/// One parse of a [`Sentence`](crate::Sentence). Its storage belongs to
/// the sentence, so it borrows the sentence, and dropping it frees
/// nothing.
pub struct Linkage<'s> {
    raw: NonNull<sys::Linkage_s>,
    sentence: &'s Sentence<'s>,
}

/// A labelled link between two words of a [`Linkage`]. `left` and
/// `right` index [`Linkage::words`].
#[derive(Clone, Copy)]
pub struct Link<'l> {
    pub left: usize,
    pub right: usize,
    /// The link's position in the linkage, as the C API numbers links.
    index: usize,
    linkage: &'l Linkage<'l>,
}

impl<'l> Link<'l> {
    /// The word at the link's left end.
    pub fn left_word(&self) -> Word<'l> { self.linkage.word(self.left) }

    /// The word at the link's right end.
    pub fn right_word(&self) -> Word<'l> { self.linkage.word(self.right) }

    /// The relation the link's [`LinkType`](crate::LinkType) names. It
    /// is `None` for every language but English.
    pub fn relation(&self) -> Option<Relation> {
        (self.linkage.dictionary().language() == Some(Language::English))
            .then(|| Relation::from_number(self.raw().link_type()))
    }

    /// The lowercase part of the link's label, such as `s*s` in `Ss*s`.
    pub fn subscript(&self) -> Subscript { self.raw().subscript() }

    /// The label link-grammar prints for the link, such as `Ss*s`.
    pub fn label(&self) -> LinkLabel<'l> {
        LinkLabel::new(self.raw().label())
    }

    /// The link's private fields.
    fn raw(&self) -> &'l link_grammar_sys::internals::Link_s {
        &self.linkage.internals().links()[self.index]
    }
}

/// The indices and label, followed by the words at the indices.
impl fmt::Debug for Link<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Link")
            .field("left", &self.left)
            .field("right", &self.right)
            .field("label", &self.label())
            .field("left_word", &self.left_word())
            .field("right_word", &self.right_word())
            .finish()
    }
}

impl<'s> Linkage<'s> {
    /// Wraps a handle from `linkage_create` for `sentence`. The borrow
    /// keeps `sentence` from being reparsed while the linkage lives.
    ///
    /// # Safety
    ///
    /// `raw` came from `linkage_create` for `sentence`.
    pub(crate) unsafe fn from_raw(
        raw: NonNull<sys::Linkage_s>,
        sentence: &'s Sentence<'s>,
    ) -> Self {
        Self { raw, sentence }
    }

    /// The words of the linkage, left wall first.
    pub fn words(&self) -> impl ExactSizeIterator<Item = Word<'_>> {
        let count = unsafe { sys::linkage_get_num_words(self.as_raw()) };
        (0..count).map(|index| self.word(index))
    }

    /// The links of the linkage.
    pub fn links(&self) -> impl ExactSizeIterator<Item = Link<'_>> {
        self.internals()
            .links()
            .iter()
            .enumerate()
            .map(|(index, raw)| Link {
                left: raw.lw.into(),
                right: raw.rw.into(),
                index,
                linkage: self,
            })
    }

    /// The ASCII diagram link-parser prints, wrapped to `width` columns.
    pub fn diagram(&self, width: usize) -> String {
        let raw = unsafe {
            sys::linkage_print_diagram(self.as_raw(), false, width)
        };
        // SAFETY: `linkage_print_diagram` returns a NUL-terminated string,
        // which is freed once, after the copy.
        let diagram = unsafe { raw.cast_const().as_utf8() }.to_owned();
        unsafe { sys::linkage_free_diagram(raw) };
        diagram
    }

    /// The word at `index`, which is below the word count.
    fn word(&self, index: usize) -> Word<'_> { Word::new(index, self) }

    /// The printed form of the word at `index`, which is below the word
    /// count.
    pub(crate) fn printed_word(&self, index: usize) -> &str {
        // SAFETY: `index` is below the word count, and the word lives as
        // long as the linkage.
        unsafe { sys::linkage_get_word(self.as_raw(), index).as_utf8() }
    }

    /// The dictionary the sentence was parsed with.
    pub(crate) fn dictionary(&self) -> &'s Dictionary {
        self.sentence.dictionary()
    }

    /// The handle the C API takes.
    pub(crate) fn as_raw(&self) -> sys::Linkage { self.raw.as_ptr() }
}
