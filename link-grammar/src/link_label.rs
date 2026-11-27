use crate::Subscript;

/// A link's label, such as `MVp`, which is the [`LinkType`] `MV` followed
/// by the [`Subscript`](crate::Subscript) `p`.
///
/// Labels only come from a parse, so each one has link-grammar's shape.
/// The link type is uppercase letters and `_`, and the subscript is
/// lowercase letters, digits and `*`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LinkLabel<'l>(&'l str);

/// The uppercase part of a [`LinkLabel`], such as `MV`.
/// [`Link::relation`](crate::Link::relation) reads it as a grammatical
/// [`Relation`](crate::Relation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LinkType<'l>(&'l str);

impl<'l> LinkLabel<'l> {
    /// Wraps a label that link-grammar computed.
    pub(crate) fn new(label: &'l str) -> Self { Self(label) }

    /// The uppercase part, such as `MV` in `MVp`.
    pub fn link_type(self) -> LinkType<'l> { LinkType(self.split().0) }

    /// The lowercase part, such as `p` in `MVp`.
    pub fn subscript(self) -> Subscript {
        Subscript::from_label(self.split().1)
    }

    /// The label split where its subscript starts.
    fn split(self) -> (&'l str, &'l str) {
        let subscript_start = self
            .0
            .find(|c: char| !(c.is_ascii_uppercase() || c == '_'))
            .unwrap_or(self.0.len());
        self.0.split_at(subscript_start)
    }
}

borrowed_str!(LinkLabel.0, LinkType.0);
