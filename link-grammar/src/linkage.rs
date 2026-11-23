use std::ffi::CStr;
use std::fmt;
use std::marker::PhantomData;
use std::ptr::{self, NonNull};

use link_grammar_sys as sys;

use crate::c_str_ext::CStrPtr;
use crate::{Language, LinkLabel, Relation, Word};

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
///     println!("{} -{}- {}", link.left_word(), link.label, link.right_word());
/// }
/// # Ok::<(), link_grammar::Error>(())
/// ```
pub struct Linkage<'s> {
    raw: NonNull<sys::Linkage_s>,
    /// The language of the sentence's dictionary.
    language: Option<Language>,
    sentence: PhantomData<&'s ()>,
}

/// A labelled link between two words of a [`Linkage`]. `left` and
/// `right` are indices into [`Linkage::words`], and
/// [`left_word`](Self::left_word) and [`right_word`](Self::right_word)
/// look up the words at them. [`relation`](Self::relation) names what
/// the label means.
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
    pub label: LinkLabel<'l>,
    linkage: &'l Linkage<'l>,
}

impl<'l> Link<'l> {
    /// The word at the link's left end.
    pub fn left_word(&self) -> Word<'l> { self.linkage.word(self.left) }

    /// The word at the link's right end.
    pub fn right_word(&self) -> Word<'l> { self.linkage.word(self.right) }

    /// The relation the label's link type names, as
    /// [`LinkType::relation`](crate::LinkType::relation) gives it.
    pub fn relation(&self) -> Option<Relation> {
        self.label.link_type().relation()
    }
}

/// The indices and label, followed by the words at the indices.
impl fmt::Debug for Link<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Link")
            .field("left", &self.left)
            .field("right", &self.right)
            .field("label", &self.label)
            .field("left_word", &self.left_word())
            .field("right_word", &self.right_word())
            .finish()
    }
}

/// Links are equal when they join the same words of the same linkage
/// with the same label.
impl PartialEq for Link<'_> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.linkage, other.linkage)
            && self.left == other.left
            && self.right == other.right
            && self.label == other.label
    }
}

impl Eq for Link<'_> {}

impl<'s> Linkage<'s> {
    /// Wraps a handle from `linkage_create` for a sentence parsed with a
    /// dictionary of `language`.
    ///
    /// # Safety
    ///
    /// `raw` came from `linkage_create` for a sentence that outlives `'s`
    /// and isn't reparsed during it.
    pub(crate) unsafe fn from_raw(
        raw: NonNull<sys::Linkage_s>,
        language: Option<Language>,
    ) -> Self {
        Self {
            raw,
            language,
            sentence: PhantomData,
        }
    }

    /// The words of the linkage, left wall first.
    pub fn words(&self) -> impl ExactSizeIterator<Item = Word<'_>> {
        // SAFETY: `raw` is a live linkage.
        let count = unsafe { sys::linkage_get_num_words(self.as_raw()) };
        (0..count).map(|index| self.word(index))
    }

    /// The links of the linkage.
    pub fn links(&self) -> impl ExactSizeIterator<Item = Link<'_>> {
        // SAFETY: `raw` is a live linkage.
        let count = unsafe { sys::linkage_get_num_links(self.as_raw()) };
        (0..count).map(|index| self.link_at(index))
    }

    /// The first link whose label names `relation`. Use
    /// [`links`](Self::links) to find every such link.
    ///
    /// ```
    /// # use link_grammar::{Dictionary, Language, Relation, Sentence};
    /// # let dictionary = Dictionary::new(Language::English)?;
    /// # let mut sentence = Sentence::new(&dictionary, "Mars is red.")?;
    /// sentence.parse()?;
    /// let linkage = sentence.linkage(0).expect("a parse");
    /// let subject = linkage.link(Relation::Subject).expect("a subject link");
    /// assert_eq!(subject.left_word().text(), "Mars");
    /// assert!(linkage.link(Relation::Object).is_none());
    /// # Ok::<(), link_grammar::Error>(())
    /// ```
    pub fn link(&self, relation: Relation) -> Option<Link<'_>> {
        self.links().find(|link| link.relation() == Some(relation))
    }

    /// The ASCII diagram link-parser prints, wrapped to `width` columns.
    pub fn diagram(&self, width: usize) -> Diagram<'_> {
        Diagram {
            linkage: self,
            width,
        }
    }

    /// The link at `index`, which is below the link count.
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
        // SAFETY: `index` is below the link count, and the label lives as
        // long as the linkage.
        let label = unsafe {
            LinkLabel::new(
                sys::linkage_get_link_label(raw, index).as_utf8(),
                self.language,
            )
        };
        Link {
            left,
            right,
            label,
            linkage: self,
        }
    }

    /// The word at `index`, which is below the word count.
    fn word(&self, index: usize) -> Word<'_> {
        // SAFETY: `index` is below the word count, and the word lives as
        // long as the linkage.
        let printed = unsafe {
            sys::linkage_get_word(self.as_raw(), index).as_utf8()
        };
        Word::new(printed, index, self)
    }

    /// The language of the sentence's dictionary.
    pub(crate) fn language(&self) -> Option<Language> { self.language }

    /// The handle the C API takes.
    fn as_raw(&self) -> sys::Linkage { self.raw.as_ptr() }
}

/// The ASCII diagram of a [`Linkage`], wrapped to a column width. Its
/// [`Display`](fmt::Display) prints what link-parser prints.
///
/// ```
/// # use link_grammar::{Dictionary, Language, Sentence};
/// # let dictionary = Dictionary::new(Language::English)?;
/// # let mut sentence = Sentence::new(&dictionary, "Mars is red.")?;
/// sentence.parse()?;
/// let linkage = sentence.linkage(0).expect("a parse");
/// println!("{}", linkage.diagram(80));
/// # Ok::<(), link_grammar::Error>(())
/// ```
pub struct Diagram<'l> {
    linkage: &'l Linkage<'l>,
    width: usize,
}

impl fmt::Display for Diagram<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // SAFETY: `linkage` is a live linkage.
        let raw = unsafe {
            sys::linkage_print_diagram(
                self.linkage.as_raw(),
                false,
                self.width,
            )
        };
        // SAFETY: `linkage_print_diagram` returns a NUL-terminated string.
        let result =
            f.write_str(&unsafe { CStr::from_ptr(raw) }.to_string_lossy());
        // SAFETY: `raw` came from `linkage_print_diagram` and is freed
        // once.
        unsafe { sys::linkage_free_diagram(raw) };
        result
    }
}
