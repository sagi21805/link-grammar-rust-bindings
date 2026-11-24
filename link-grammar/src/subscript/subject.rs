//! The named letters of an `S` link.

use super::{Subscript, SubscriptLetter};
use crate::{Relation, RelationSubscript};

/// The subscript of an `S` link, such as `Ss*t`.
///
/// ```
/// use link_grammar::subscript::{Subject, SubjectKind};
/// use link_grammar::{Dictionary, Language, Sentence};
///
/// let dictionary = Dictionary::new(Language::English)?;
/// let mut sentence = Sentence::new(&dictionary, "The dogs run.")?;
/// sentence.parse()?;
/// let linkage = sentence.linkage(0).expect("a parse");
/// let (_, subject) =
///     linkage.links_as::<Subject>().next().expect("a subject link");
/// assert_eq!(subject.kind, Some(SubjectKind::Plural));
/// # Ok::<(), link_grammar::Error>(())
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Subject {
    /// Position 0.
    pub kind: Option<SubjectKind>,
    /// Position 1.
    pub flag: Option<SubjectFlag>,
    /// Position 2.
    pub constraint: Option<SubjectConstraint>,
}

impl RelationSubscript for Subject {
    const RELATION: Relation = Relation::Subject;

    fn from_subscript(subscript: Subscript) -> Self {
        Self {
            kind: SubjectKind::from_letter(subscript.letter(0)),
            flag: SubjectFlag::from_letter(subscript.letter(1)),
            constraint: SubjectConstraint::from_letter(
                subscript.letter(2),
            ),
        }
    }
}

subscript_letters! {
    /// Position 0 of an `S` link.
    pub enum SubjectKind {
        /// `Ss`, a singular noun with a singular verb. "The dog runs."
        Singular = b's',
        /// `Sp`, a plural noun with a plural verb. "The dogs run."
        Plural = b'p',
        /// `Su`, an uncountable noun.
        Uncountable = b'u',
        /// `Sa`, a past participle after "as". "He left as agreed."
        As = b'a',
        /// `Sj`, the subject of a bare infinitive. "You should hear him
        /// talk."
        BareInfinitive = b'j',
        /// `Sg`, the subject of a gerund. "I feel him breathing down my
        /// back."
        Gerund = b'g',
    }
}

subscript_letters! {
    /// Position 1 of an `S` link.
    pub enum SubjectFlag {
        /// `Spx` and `Ssx`, which the dictionary uses for the plural
        /// subject of "be" ("They are getting hot") and for passive
        /// participles (`S*x`).
        Extended = b'x',
    }
}

subscript_letters! {
    /// Position 2 of an `S` link.
    pub enum SubjectConstraint {
        /// `Ss*t`, a subject that takes a "be that" predicate.
        BeThat = b't',
        /// `Ss*w`, a question word as the subject. "Who am I?"
        Question = b'w',
    }
}
