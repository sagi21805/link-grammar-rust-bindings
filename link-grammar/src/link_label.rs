use std::fmt;

use crate::{LinkRelation, Subscript, english};

/// A link's label, such as `MVp`, which is the relation `MV` and the
/// subscript `p`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LinkLabel<R: LinkRelation = english::Relation> {
    /// The label's uppercase part, such as `MV`.
    pub relation: R,
    /// The label's lowercase part, such as `p`.
    pub subscript: Subscript,
}

/// `MVp`, as link-grammar prints it. Idioms print as `_I`.
impl<R: LinkRelation> fmt::Display for LinkLabel<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.relation.code(), self.subscript)
    }
}
