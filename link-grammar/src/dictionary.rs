use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr::NonNull;
use std::sync::Mutex;

use link_grammar_sys as sys;

use crate::{Error, Language, Result};

/// The grammar and word list of one language.
///
/// [`Sentence`](crate::Sentence)s borrow the dictionary they are parsed
/// with, so it outlives all of them.
///
/// ```no_run
/// use std::path::Path;
///
/// use link_grammar::{Dictionary, Language};
///
/// Dictionary::set_data_dir(Path::new("link-grammar/data"))?;
/// let russian = Dictionary::new(Language::Russian)?;
/// # Ok::<(), link_grammar::Error>(())
/// ```
// TODO: Upstream allows sharing one dictionary between threads. Add
// `Send` and `Sync` once something needs it.
pub struct Dictionary {
    raw: NonNull<sys::Dictionary_s>,
}

impl Dictionary {
    /// Loads the dictionary for `language`.
    ///
    /// link-grammar searches the directory given to
    /// [`set_data_dir`](Self::set_data_dir), then the dictionaries this
    /// crate ships, then `./data`.
    pub fn new(language: Language) -> Result<Self> {
        Self::create(language.c_code(), || language.code().to_owned())
    }

    /// Loads the dictionary in the directory `path`, which holds
    /// `4.0.dict` and the files it includes.
    ///
    /// ```no_run
    /// use std::path::Path;
    ///
    /// use link_grammar::Dictionary;
    ///
    /// let custom = Dictionary::from_path(Path::new("my-dictionaries/en"))?;
    /// # Ok::<(), link_grammar::Error>(())
    /// ```
    pub fn from_path(path: &Path) -> Result<Self> {
        let c_path = CString::new(path.as_os_str().as_encoded_bytes())?;
        Self::create(&c_path, || path.display().to_string())
    }

    /// Loads the dictionary that `dictionary_create_lang` finds for
    /// `name`, a language code or a path. `describe` names it in the
    /// error.
    fn create(
        name: &CStr,
        describe: impl FnOnce() -> String,
    ) -> Result<Self> {
        let _guard = DATA_DIR_LOCK.lock().expect("no panic while locked");
        // SAFETY: `name` is NUL-terminated and outlives the call.
        let raw = unsafe { sys::dictionary_create_lang(name.as_ptr()) };
        NonNull::new(raw)
            .map(|raw| Self { raw })
            .ok_or_else(|| Error::DictionaryNotFound(describe()))
    }

    /// Makes later [`new`](Self::new) calls look in `path` first. `path`
    /// holds one subdirectory per language.
    pub fn set_data_dir(path: &Path) -> Result<()> {
        let c_path = CString::new(path.as_os_str().as_encoded_bytes())?;
        let _guard = DATA_DIR_LOCK.lock().expect("no panic while locked");
        // SAFETY: `c_path` is NUL-terminated and link-grammar copies it.
        unsafe { sys::dictionary_set_data_dir(c_path.as_ptr()) };
        Ok(())
    }

    /// The handle the C API takes.
    pub(crate) fn as_raw(&self) -> sys::Dictionary { self.raw.as_ptr() }
}

impl Drop for Dictionary {
    fn drop(&mut self) {
        // SAFETY: `raw` came from `dictionary_create_lang`, and every
        // sentence borrowing it is already gone.
        unsafe { sys::dictionary_delete(self.raw.as_ptr()) };
    }
}

/// Serializes access to link-grammar's global data directory, which
/// `dictionary_set_data_dir` writes and `dictionary_create_lang` reads.
static DATA_DIR_LOCK: Mutex<()> = Mutex::new(());
