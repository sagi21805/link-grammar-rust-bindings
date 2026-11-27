//! English, the language the crate ships a dictionary for.

mod relation;

pub use relation::Relation;

use crate::Language;

/// English. A marker type that holds no data.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct English;

impl Language for English {
    type Relation = Relation;

    const CODE: &'static str = "en";
}

#[cfg(test)]
mod tests {
    use super::English;

    #[test]
    fn readers_agree_with_the_c_getters() {
        crate::differential_tests::check_corpus::<English>();
    }
}
