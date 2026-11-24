//! Reads a link's subscript as a struct of named letters.

use crate::{Link, Linkage, Relation, Subscript};

/// A struct that names the letters of one relation's subscripts.
pub trait RelationSubscript: Sized {
    /// The relation whose links use this layout.
    const RELATION: Relation;

    /// Names the letters of `subscript`. A position whose letter has no
    /// name reads `None`, as a `*` does.
    fn from_subscript(subscript: Subscript) -> Self;
}

impl Link<'_> {
    /// The link's subscript with its letters named, or `None` when the
    /// link isn't a `T::RELATION` link.
    pub fn subscript_as<T: RelationSubscript>(&self) -> Option<T> {
        (self.relation()? == T::RELATION)
            .then(|| T::from_subscript(self.subscript()))
    }
}

impl Linkage<'_> {
    /// The `T::RELATION` links, each with its letters named.
    pub fn links_as<T: RelationSubscript>(
        &self,
    ) -> impl Iterator<Item = (Link<'_>, T)> {
        self.links()
            .filter_map(|link| Some((link, link.subscript_as::<T>()?)))
    }
}
