//! Readers over link-grammar's private structs, which the bundled build
//! knows the layouts of. Every pointer walk into them is here.

use std::slice;

use extend::ext;
use link_grammar_sys::internals;

use crate::c_str_ext::CStrPtr;
use crate::link_types::LinkTypeId;
use crate::{Dictionary, Linkage, Subscript};

impl Linkage<'_> {
    /// The linkage's private fields.
    pub(crate) fn internals(&self) -> &internals::Linkage_s {
        // SAFETY: `raw` came from `linkage_create`, so it points to a full
        // `Linkage_s`, which lives as long as `self`.
        unsafe { &*self.as_raw().cast() }
    }
}

impl Dictionary {
    /// The dictionary's private fields.
    pub(crate) fn internals(&self) -> &internals::Dictionary_s {
        // SAFETY: `raw` came from `dictionary_create_lang`, so it points
        // to a full `Dictionary_s`, which lives as long as `self`.
        unsafe { &*self.as_raw().cast() }
    }
}

#[ext(name = LinkageInternals)]
pub(crate) impl internals::Linkage_s {
    /// The links, in the order the `linkage_get_link_*` getters number
    /// them.
    fn links(&self) -> &[internals::Link_s] {
        // SAFETY: `link_array` holds `num_links` links, which nothing
        // writes after `linkage_create`.
        unsafe {
            slice::from_raw_parts(self.link_array, self.num_links as usize)
        }
    }
}

#[ext(name = LinkInternals)]
pub(crate) impl internals::Link_s {
    /// The number of the link's uppercase part, which both ends share.
    fn link_type(&self) -> LinkTypeId {
        LinkTypeId::new(self.left_end().uc_num)
    }

    /// The OR of both ends' subscripts, the way `intersect_strings`
    /// computes the label.
    fn subscript(&self) -> Subscript {
        let letters =
            |end: &internals::condesc_struct| end.lc_letters >> 1;
        Subscript::from_bits(
            letters(self.left_end()) | letters(self.right_end()),
        )
    }

    /// The label, such as `Ss*s`.
    fn label(&self) -> &str {
        // SAFETY: `link_name` is a NUL-terminated string in the sentence's
        // string set, which outlives the linkage.
        unsafe { self.link_name.as_utf8() }
    }

    /// The descriptor of the link's left connector.
    fn left_end(&self) -> &internals::condesc_struct {
        // SAFETY: a link's connectors belong to the sentence, and their
        // descriptors to the dictionary.
        unsafe { &*(*self.lc).desc }
    }

    /// The descriptor of the link's right connector.
    fn right_end(&self) -> &internals::condesc_struct {
        // SAFETY: as in `left_end`.
        unsafe { &*(*self.rc).desc }
    }
}

#[ext(name = DictionaryInternals)]
pub(crate) impl internals::Dictionary_s {
    /// Every connector descriptor the dictionary has.
    fn connectors(
        &self,
    ) -> impl Iterator<Item = &internals::condesc_struct> {
        let table = &self.contable;
        // SAFETY: `hdesc` has `size` slots, each NULL or a descriptor that
        // lives as long as the dictionary.
        let slots =
            unsafe { slice::from_raw_parts(table.hdesc, table.size) };
        // SAFETY: as above.
        slots
            .iter()
            .filter_map(|slot| unsafe { slot.desc.as_ref() })
    }
}

#[ext(name = ConnectorDescriptor)]
pub(crate) impl internals::condesc_struct {
    /// The connector's uppercase part, such as `MV` in `MVp`.
    fn link_type_name(&self) -> &str {
        // SAFETY: `more` and its `string` live as long as the dictionary.
        let more = unsafe { &*self.more };
        let start = usize::from(more.uc_start);
        // SAFETY: as above, and the string is NUL-terminated.
        let name = unsafe { more.string.as_utf8() };
        &name[start..start + usize::from(more.uc_length)]
    }
}
