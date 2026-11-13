use std::ffi::CStr;
use std::fmt;

/// A language that link-grammar has a dictionary for.
///
/// Only [`English`](Self::English) ships with this crate. The others need
/// their dictionary directory from link-grammar's `data/`, which
/// [`Dictionary::set_data_dir`](crate::Dictionary::set_data_dir) points
/// at.
///
/// ```
/// use link_grammar::{Dictionary, Language};
///
/// let english = Dictionary::new(Language::English)?;
/// assert_eq!(Language::English.code(), "en");
/// # Ok::<(), link_grammar::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Language {
    Arabic,
    English,
    German,
    Hebrew,
    Indonesian,
    Kazakh,
    Lithuanian,
    Persian,
    Russian,
    Thai,
    Turkish,
    Vietnamese,
}

impl Language {
    /// Every language, in the order the enum declares them.
    const ALL: [Self; 12] = [
        Self::Arabic,
        Self::English,
        Self::German,
        Self::Hebrew,
        Self::Indonesian,
        Self::Kazakh,
        Self::Lithuanian,
        Self::Persian,
        Self::Russian,
        Self::Thai,
        Self::Turkish,
        Self::Vietnamese,
    ];

    /// The language whose [`code`](Self::code) is `code`, if there is
    /// one.
    pub(crate) fn from_code(code: &CStr) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|language| language.c_code() == code)
    }

    /// The name of the language's dictionary directory, such as `"en"`.
    pub fn code(self) -> &'static str {
        self.c_code().to_str().expect("language codes are ASCII")
    }

    /// [`code`](Self::code) as the C string `dictionary_create_lang`
    /// takes.
    pub(crate) fn c_code(self) -> &'static CStr {
        match self {
            Self::Arabic => c"ar",
            Self::English => c"en",
            Self::German => c"de",
            Self::Hebrew => c"he",
            Self::Indonesian => c"id",
            Self::Kazakh => c"kz",
            Self::Lithuanian => c"lt",
            Self::Persian => c"fa",
            Self::Russian => c"ru",
            Self::Thai => c"th",
            Self::Turkish => c"tr",
            Self::Vietnamese => c"vn",
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}
