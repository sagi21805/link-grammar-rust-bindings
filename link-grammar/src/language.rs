use std::fmt::Debug;
use std::hash::Hash;

/// A language link-grammar can parse, as a marker type. The supertraits
/// let types that carry a language derive their own traits.
///
/// Only [`English`](crate::english::English) ships with this crate.
/// Another language needs its dictionary directory from link-grammar's
/// `data/`, which [`set_data_dir`](crate::set_data_dir) points at.
pub trait Language: Copy + Default + Eq + Hash + Debug + 'static {
    /// The relations the language's link types name.
    type Relation: LinkRelation;
    /// The name of the dictionary's directory, such as `en`.
    const CODE: &'static str;
}

/// The relations one language's link types name, one variant per link
/// type number.
pub trait LinkRelation: Copy + Eq + Hash + Debug + 'static {
    /// The variant for a link type number of the language's dictionary.
    fn from_number(number: u32) -> Self;
    /// The link type, such as `MV`, or `_I` for an idiom.
    fn code(self) -> &'static str;
}
