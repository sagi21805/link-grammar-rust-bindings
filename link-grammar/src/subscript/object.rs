//! The named letters of an `O` link.

use super::{Subscript, SubscriptLetter};
use crate::{Relation, RelationSubscript};

/// The subscript of an `O` link, such as `Os` or `O*n`.
///
/// ```
/// use link_grammar::subscript::{Object, ObjectKind};
/// use link_grammar::{Dictionary, Language, Sentence};
///
/// let dictionary = Dictionary::new(Language::English)?;
/// let mut sentence = Sentence::new(&dictionary, "The rover drilled the rock.")?;
/// sentence.parse()?;
/// let linkage = sentence.linkage(0).expect("a parse");
/// let (_, object) =
///     linkage.links_as::<Object>().next().expect("an object link");
/// assert_eq!(object.kind, Some(ObjectKind::Singular));
/// # Ok::<(), link_grammar::Error>(())
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Object {
    /// Position 0.
    pub kind: Option<ObjectKind>,
    /// Position 1.
    pub flag: Option<ObjectFlag>,
    /// Position 2.
    pub usage: Option<ObjectUse>,
}

impl RelationSubscript for Object {
    const RELATION: Relation = Relation::Object;

    fn from_subscript(subscript: Subscript) -> Self {
        Self {
            kind: ObjectKind::from_letter(subscript.letter(0)),
            flag: ObjectFlag::from_letter(subscript.letter(1)),
            usage: ObjectUse::from_letter(subscript.letter(2)),
        }
    }
}

subscript_letters! {
    /// Position 0 of an `O` link.
    pub enum ObjectKind {
        /// `Os`, a singular object.
        Singular = b's',
        /// `Op`, a plural object. "I want only these."
        Plural = b'p',
    }
}

subscript_letters! {
    /// Position 1 of an `O` link.
    pub enum ObjectFlag {
        /// `O*m`, which allows "If only there were more!"
        More = b'm',
        /// `O*x`, where the `x` blocks ditransitive verbs. "I should
        /// imagine not!"
        BlocksDitransitive = b'x',
    }
}

subscript_letters! {
    /// Position 2 of an `O` link.
    pub enum ObjectUse {
        /// `Os*e`, which allows certain adjectival uses.
        Adjectival = b'e',
    }
}
