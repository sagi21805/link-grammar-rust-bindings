use std::fmt;

/// A link's label, such as `MVp`, which is the [`LinkType`] `MV` followed
/// by the [`Subscript`] `p`.
///
/// Labels only come from a parse, so each one has link-grammar's shape.
/// The link type is uppercase letters and `_`, and the subscript is
/// lowercase letters, digits and `*`. A label borrows the linkage's
/// string, and splitting it scans only the label's few bytes.
///
/// ```
/// use link_grammar::{Dictionary, Language, ParseOptions, Sentence};
///
/// let dictionary = Dictionary::new(Language::English)?;
/// let options = ParseOptions::new();
/// let mut sentence = Sentence::new(&dictionary, "Mars is red.")?;
/// sentence.parse(&options)?;
/// let linkage = sentence.linkage(0, &options).expect("a parse");
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
pub struct LinkLabel<'l>(&'l str);

/// The uppercase part of a [`LinkLabel`], such as `MV`, which names the
/// grammatical relation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LinkType<'l>(&'l str);

/// The lowercase part of a [`LinkLabel`], such as `p`, which narrows its
/// [`LinkType`]. Each `*` matches any letter in that position. It is empty
/// when the label is only a link type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Subscript<'l>(&'l str);

impl<'l> LinkLabel<'l> {
    /// Wraps a label that link-grammar computed.
    pub(crate) fn new(label: &'l str) -> Self {
        let label = Self(label);
        debug_assert!(
            label.is_well_formed(),
            "link-grammar returned the malformed label {label}"
        );
        label
    }

    /// The uppercase part, such as `MV` in `MVp`.
    pub fn link_type(self) -> LinkType<'l> { LinkType(self.split().0) }

    /// The lowercase part, such as `p` in `MVp`.
    pub fn subscript(self) -> Subscript<'l> { Subscript(self.split().1) }

    /// The label split where its subscript starts.
    fn split(self) -> (&'l str, &'l str) {
        let subscript_start = self
            .0
            .find(|c| !is_link_type_char(c))
            .unwrap_or(self.0.len());
        self.0.split_at(subscript_start)
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

/// `as_str`, `Display` and comparisons with strings for the borrowed
/// string types of this module.
macro_rules! label_str {
    ($($ty:ident),*) => {$(
        impl<'l> $ty<'l> {
            #[doc = concat!("The ", stringify!($ty), " as a string.")]
            pub fn as_str(self) -> &'l str { self.0 }
        }

        impl fmt::Display for $ty<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.0)
            }
        }

        impl PartialEq<str> for $ty<'_> {
            fn eq(&self, other: &str) -> bool { self.0 == other }
        }

        impl PartialEq<&str> for $ty<'_> {
            fn eq(&self, other: &&str) -> bool { self.0 == *other }
        }
    )*};
}

label_str!(LinkLabel, LinkType, Subscript);

// The label types are a `&str` and nothing more.
const _: () = {
    use std::mem::size_of;
    assert!(size_of::<LinkLabel<'_>>() == size_of::<&str>());
    assert!(size_of::<LinkType<'_>>() == size_of::<&str>());
    assert!(size_of::<Subscript<'_>>() == size_of::<&str>());
};

/// Whether `c` belongs in a link type.
fn is_link_type_char(c: char) -> bool {
    c.is_ascii_uppercase() || c == '_'
}
