use std::fmt;
use std::ptr::NonNull;

use link_grammar_sys as sys;

use crate::c_str_ext::CStrPtr;
#[cfg(feature = "bundled")]
use crate::internals_ext::{LinkInternals, LinkageInternals};
use crate::{
    Dictionary, Language, LinkLabel, Relation, Sentence, Subscript, Word,
};

/// One parse of a [`Sentence`](crate::Sentence). Its storage belongs to
/// the sentence, so it borrows the sentence, and dropping it frees
/// nothing.
///
/// ```
/// # use link_grammar::{Dictionary, Language, Sentence};
/// # let dictionary = Dictionary::new(Language::English)?;
/// # let mut sentence = Sentence::new(&dictionary, "Mars is red.")?;
/// sentence.parse()?;
/// let linkage = sentence.linkage(0).expect("a parse");
/// for link in linkage.links() {
///     println!("{} -{}- {}", link.left_word(), link.label(), link.right_word());
/// }
/// # Ok::<(), link_grammar::Error>(())
/// ```
pub struct Linkage<'s> {
    raw: NonNull<sys::Linkage_s>,
    sentence: &'s Sentence<'s>,
}

/// A labelled link between two words of a [`Linkage`]. `left` and
/// `right` index [`Linkage::words`].
///
/// ```
/// # use link_grammar::{Dictionary, Language, Relation, Sentence};
/// # let dictionary = Dictionary::new(Language::English)?;
/// # let mut sentence = Sentence::new(&dictionary, "Mars is red.")?;
/// sentence.parse()?;
/// let linkage = sentence.linkage(0).expect("a parse");
/// let subject = linkage.link(Relation::Subject).expect("a subject link");
/// assert_eq!(subject.relation(), Some(Relation::Subject));
/// assert_eq!(subject.left_word().text(), "Mars");
/// assert_eq!(subject.right_word().text(), "is");
/// # Ok::<(), link_grammar::Error>(())
/// ```
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
        #[cfg(feature = "bundled")]
        return self
            .linkage
            .dictionary()
            .link_types()
            .relation(self.raw().link_type());
        #[cfg(not(feature = "bundled"))]
        return self.relation_from_label();
    }

    /// The relation the label's link type names, read from the printed
    /// label.
    #[cfg(not(feature = "bundled"))]
    pub(crate) fn relation_from_label(&self) -> Option<Relation> {
        match self.linkage.language() {
            Some(Language::English) => Relation::from_english_link_type(
                self.label().link_type().as_str(),
            ),
            _ => None,
        }
    }

    /// The lowercase part of the link's label, such as `s*s` in `Ss*s`.
    pub fn subscript(&self) -> Subscript {
        #[cfg(feature = "bundled")]
        return self.raw().subscript();
        #[cfg(not(feature = "bundled"))]
        return self.label().subscript();
    }

    /// The label link-grammar prints for the link, such as `Ss*s`.
    pub fn label(&self) -> LinkLabel<'l> {
        #[cfg(feature = "bundled")]
        return LinkLabel::new(self.raw().label());
        #[cfg(not(feature = "bundled"))]
        // SAFETY: `index` is below the link count, and the label lives
        // as long as the linkage.
        return LinkLabel::new(unsafe {
            sys::linkage_get_link_label(self.linkage.as_raw(), self.index)
                .as_utf8()
        });
    }

    /// The link's private fields.
    #[cfg(feature = "bundled")]
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

/// Links are equal when they join the same positions with the same link
/// type and subscript, so a link equals itself in another linkage of the
/// same parse.
impl PartialEq for Link<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left
            && self.right == other.right
            && self.subscript() == other.subscript()
            && self.has_link_type_of(other)
    }
}

impl Link<'_> {
    /// Whether the two links have the same link type.
    fn has_link_type_of(&self, other: &Self) -> bool {
        #[cfg(feature = "bundled")]
        return self.raw().link_type() == other.raw().link_type();
        #[cfg(not(feature = "bundled"))]
        return self.label().link_type() == other.label().link_type();
    }
}

impl Eq for Link<'_> {}

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
    #[cfg(feature = "bundled")]
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

    /// The links of the linkage.
    #[cfg(not(feature = "bundled"))]
    pub fn links(&self) -> impl ExactSizeIterator<Item = Link<'_>> {
        let count = unsafe { sys::linkage_get_num_links(self.as_raw()) };
        (0..count).map(|index| self.link_at(index))
    }

    /// The first link whose label names `relation`. Use
    /// [`links`](Self::links) to find every such link.
    pub fn link(&self, relation: Relation) -> Option<Link<'_>> {
        self.links().find(|link| link.relation() == Some(relation))
    }

    pub fn link_with_subscript_start(
        &self,
        r: Relation,
        s: char,
    ) -> Option<Link<'_>> {
        self.links().find(|l| {
            l.relation() == (Some(r))
                && l.subscript().get(0).map(|c| c == s).unwrap_or(false)
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

    /// The link at `index`, which is below the link count.
    #[cfg(not(feature = "bundled"))]
    fn link_at(&self, index: usize) -> Link<'_> {
        let raw = self.as_raw();
        // SAFETY: `index` is below the link count, so the C API returns
        // the indices of words in the linkage.
        let (left, right) = unsafe {
            (
                sys::linkage_get_link_lword(raw, index),
                sys::linkage_get_link_rword(raw, index),
            )
        };
        Link {
            left,
            right,
            index,
            linkage: self,
        }
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

    /// The language of the sentence's dictionary.
    pub(crate) fn language(&self) -> Option<Language> {
        self.dictionary().language()
    }

    /// The dictionary the sentence was parsed with.
    pub(crate) fn dictionary(&self) -> &'s Dictionary {
        self.sentence.dictionary()
    }

    /// The handle the C API takes.
    pub(crate) fn as_raw(&self) -> sys::Linkage { self.raw.as_ptr() }
}
