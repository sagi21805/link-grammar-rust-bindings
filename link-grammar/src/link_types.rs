//! The relation each link type number of a dictionary names.

use link_grammar_sys::internals;

use crate::internals_ext::{ConnectorDescriptor, DictionaryInternals};
use crate::{Language, Relation};

/// A number link-grammar gives the uppercase part of a link label. It
/// only means something inside the dictionary that gave it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct LinkTypeId(u32);

impl LinkTypeId {
    /// Wraps a descriptor's `uc_num`.
    pub(crate) fn new(uc_num: u32) -> Self { Self(uc_num) }
}

/// The relation each link type number of one dictionary names.
#[derive(Default)]
pub(crate) struct LinkTypes {
    relations: Box<[Option<Relation>]>,
}

impl LinkTypes {
    /// The relation of each of `dictionary`'s link types. Only English has
    /// relations, so other languages get an empty table.
    pub(crate) fn new(
        dictionary: &internals::Dictionary_s,
        language: Option<Language>,
    ) -> Self {
        if language != Some(Language::English) {
            return Self::default();
        }
        let mut relations = vec![None; dictionary.contable.num_uc];
        for connector in dictionary.connectors() {
            if let Some(relation) =
                relations.get_mut(connector.uc_num as usize)
            {
                *relation = Relation::from_english_link_type(
                    connector.link_type_name(),
                );
            }
        }
        Self {
            relations: relations.into(),
        }
    }

    /// The relation `link_type` names.
    pub(crate) fn relation(
        &self,
        link_type: LinkTypeId,
    ) -> Option<Relation> {
        *self.relations.get(link_type.0 as usize)?
    }
}
