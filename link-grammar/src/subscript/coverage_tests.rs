//! Checks that every letter the English dictionary puts in an `S` or `O`
//! connector has a name, or is listed as one that has none.

use std::collections::BTreeMap;

use super::{
    ObjectFlag, ObjectKind, ObjectUse, SubjectConstraint, SubjectFlag,
    SubjectKind, SubscriptLetter,
};
use crate::internals_ext::{ConnectorDescriptor, DictionaryInternals};
use crate::{Dictionary, Language, Relation, RelationSubscript};

/// Whether a position names a letter.
type Names = fn(u8) -> bool;

/// The letters at a position that no comment in `4.0.dict` explains. Most
/// only block links to another connector, as `Ss*s+` blocks `Ss*t-`.
/// They read `None` in the named structs, and `Link::subscript` still has
/// them.
type Unnamed = &'static [(usize, char)];

/// The `S` letters with no name.
const UNNAMED_SUBJECT: Unnamed = &[
    (2, 'a'),
    (2, 'b'),
    (2, 'c'),
    (2, 'd'),
    (2, 'g'),
    (2, 'i'),
    (2, 'o'),
    (2, 'q'),
    (2, 's'),
    (2, 'v'),
];

/// The `O` letters with no name.
const UNNAMED_OBJECT: Unnamed = &[
    (0, 'm'),
    (0, 't'),
    (0, 'u'),
    (0, 'x'),
    (0, 'y'),
    (1, 'c'),
    (1, 'i'),
    (1, 'n'),
    (1, 't'),
];

#[test]
fn every_subject_letter_has_a_name() {
    let names: &[Names] = &[
        |letter| SubjectKind::from_letter(letter).is_some(),
        |letter| SubjectFlag::from_letter(letter).is_some(),
        |letter| SubjectConstraint::from_letter(letter).is_some(),
    ];
    assert_only_known_unnamed::<super::Subject>(names, UNNAMED_SUBJECT);
}

#[test]
fn every_object_letter_has_a_name() {
    let names: &[Names] = &[
        |letter| ObjectKind::from_letter(letter).is_some(),
        |letter| ObjectFlag::from_letter(letter).is_some(),
        |letter| ObjectUse::from_letter(letter).is_some(),
    ];
    assert_only_known_unnamed::<super::Object>(names, UNNAMED_OBJECT);
}

/// Fails with every unnamed letter of `T::RELATION`'s connectors that
/// isn't in `known`, each with one connector that has it.
fn assert_only_known_unnamed<T: RelationSubscript>(
    names: &[Names],
    known: Unnamed,
) {
    let dictionary =
        Dictionary::new(Language::English).expect("English is bundled");
    let mut unnamed = BTreeMap::new();
    for connector in
        dictionary.internals().connectors().filter(|connector| {
            Relation::from_english_link_type(connector.link_type_name())
                == Some(T::RELATION)
        })
    {
        for position in 0..9 {
            let letter = connector.subscript().letter(position);
            let is_named =
                names.get(position).is_some_and(|names| names(letter));
            let is_known = known.contains(&(position, letter as char));
            if letter != 0 && !is_named && !is_known {
                unnamed
                    .entry((position, letter as char))
                    .or_insert_with(|| connector.connector().to_owned());
            }
        }
    }
    assert!(
        unnamed.is_empty(),
        "new unnamed letters of {:?}, as (position, letter): \
         connector\n{unnamed:#?}",
        T::RELATION,
    );
}
