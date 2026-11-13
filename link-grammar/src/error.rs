use std::ffi::NulError;

/// A `Result` whose error is this crate's [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// The ways creating or parsing with link-grammar can fail.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(
        "no dictionary for `{0}` was found. Point \
         `Dictionary::set_data_dir` at a directory that contains it"
    )]
    DictionaryNotFound(String),

    #[error("text passed to link-grammar must not contain a NUL byte")]
    InteriorNul(#[from] NulError),

    #[error("the sentence must split into words the dictionary knows")]
    Split,

    #[error("a sentence must have fewer words than link-grammar's limit")]
    TooLong,

    #[error(
        "generation dictionaries only parse with the default parser and \
         no null links"
    )]
    GenerationMode,
}
