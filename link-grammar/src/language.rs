use std::fmt;

coded_enum! {
    /// A language that link-grammar has a dictionary for. Its
    /// [`code`](Self::code) is the name of its dictionary directory, such
    /// as `"en"`.
    ///
    /// Only [`English`](Self::English) ships with this crate. The others
    /// need their dictionary directory from link-grammar's `data/`, which
    /// [`Dictionary::set_data_dir`](crate::Dictionary::set_data_dir)
    /// points at.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum Language {
        "ar" => Arabic,
        "en" => English,
        "de" => German,
        "he" => Hebrew,
        "id" => Indonesian,
        "kz" => Kazakh,
        "lt" => Lithuanian,
        "fa" => Persian,
        "ru" => Russian,
        "th" => Thai,
        "tr" => Turkish,
        "vn" => Vietnamese,
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}
