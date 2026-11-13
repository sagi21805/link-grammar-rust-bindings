use crate::{Language, Relation};

/// A link's label, such as `MVp`, which is the [`LinkType`] `MV` followed
/// by the [`Subscript`] `p`.
///
/// Labels only come from a parse, so each one has link-grammar's shape.
/// The link type is uppercase letters and `_`, and the subscript is
/// lowercase letters, digits and `*`. A label borrows the linkage's
/// string, and splitting it scans only the label's few bytes.
///
/// ```
/// use link_grammar::{Dictionary, Language, Sentence};
///
/// let dictionary = Dictionary::new(Language::English)?;
/// let mut sentence = Sentence::new(&dictionary, "Mars is red.")?;
/// sentence.parse()?;
/// let linkage = sentence.linkage(0).expect("a parse");
/// let subject = linkage
///     .links()
///     .map(|link| link.label)
///     .find(|label| label.link_type() == "S")
///     .expect("a subject link");
/// assert_eq!(subject, "Ss*s");
/// assert_eq!(subject.subscript(), "s*s");
/// # Ok::<(), link_grammar::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LinkLabel<'l> {
    label: &'l str,
    /// The language of the dictionary that made the link.
    language: Option<Language>,
}

/// The uppercase part of a [`LinkLabel`], such as `MV`, which names the
/// grammatical [`Relation`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LinkType<'l> {
    link_type: &'l str,
    /// The language of the dictionary that made the link.
    language: Option<Language>,
}

/// The lowercase part of a [`LinkLabel`], such as `p`, which narrows its
/// [`LinkType`]. Each `*` matches any letter in that position. It is empty
/// when the label is only a link type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Subscript<'l>(&'l str);

impl<'l> LinkLabel<'l> {
    /// Wraps a label that a dictionary of `language` computed.
    pub(crate) fn new(label: &'l str, language: Option<Language>) -> Self {
        let label = Self { label, language };
        debug_assert!(
            label.is_well_formed(),
            "link-grammar returned the malformed label {label}"
        );
        label
    }

    /// The uppercase part, such as `MV` in `MVp`.
    pub fn link_type(self) -> LinkType<'l> {
        LinkType {
            link_type: self.split().0,
            language: self.language,
        }
    }

    /// The lowercase part, such as `p` in `MVp`.
    pub fn subscript(self) -> Subscript<'l> { Subscript(self.split().1) }

    /// The label split where its subscript starts.
    fn split(self) -> (&'l str, &'l str) {
        let subscript_start = self
            .label
            .find(|c| !is_link_type_char(c))
            .unwrap_or(self.label.len());
        self.label.split_at(subscript_start)
    }

    /// Whether the label is a link type followed by a subscript.
    fn is_well_formed(self) -> bool {
        let (link_type, subscript) = self.split();
        !link_type.is_empty()
            && subscript.chars().all(|c| {
                c.is_ascii_lowercase() || c.is_ascii_digit() || c == '*'
            })
    }
}

impl LinkType<'_> {
    /// The grammatical relation the link type names. It is `None` for
    /// every language but English.
    ///
    /// ```
    /// use link_grammar::{Dictionary, Language, Relation, Sentence};
    ///
    /// let dictionary = Dictionary::new(Language::English)?;
    /// let mut sentence = Sentence::new(&dictionary, "Mars is red.")?;
    /// sentence.parse()?;
    /// let linkage = sentence.linkage(0).expect("a parse");
    /// let words: Vec<_> = linkage.words().collect();
    /// let subject = linkage
    ///     .links()
    ///     .find(|link| link.label.link_type().relation() == Some(Relation::Subject))
    ///     .expect("a subject link");
    /// assert_eq!(words[subject.left].text(), "Mars");
    /// # Ok::<(), link_grammar::Error>(())
    /// ```
    pub fn relation(self) -> Option<Relation> {
        match self.language {
            Some(Language::English) => {
                Relation::from_english_link_type(self.link_type)
            }
            _ => None,
        }
    }
}

borrowed_str!(LinkLabel.label, LinkType.link_type, Subscript.0);

/// Whether `c` belongs in a link type.
fn is_link_type_char(c: char) -> bool {
    c.is_ascii_uppercase() || c == '_'
}
