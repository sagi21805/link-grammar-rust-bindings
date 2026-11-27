use std::ffi::{CString, OsStr};
use std::marker::PhantomData;
use std::path::Path;
use std::ptr::NonNull;
use std::sync::Mutex;

use link_grammar_sys as sys;

use crate::english::English;
use crate::internals_ext::{ConnectorDescriptor, DictionaryInternals};
use crate::{Error, Language, LinkRelation, Result};

/// The grammar and word list of one language.
///
/// [`Sentence`](crate::Sentence)s borrow the dictionary they are parsed
/// with, so it outlives all of them.
// TODO: Upstream allows sharing one dictionary between threads. Add
// `Send` and `Sync` once something needs it.
pub struct Dictionary<L: Language = English> {
    raw: NonNull<sys::Dictionary_s>,
    language: PhantomData<L>,
}

impl<L: Language> Dictionary<L> {
    /// Loads the dictionary for `L`. It takes the marker value, so `L` is
    /// inferred at the call site.
    ///
    /// link-grammar searches the directory given to [`set_data_dir`], then
    /// the dictionaries this crate ships, then `./data`.
    pub fn new(_language: L) -> Result<Self> {
        Self::create(OsStr::new(L::CODE))
    }

    /// Loads the dictionary that `dictionary_create_lang` finds for
    /// `name`, a language code.
    fn create(name: &OsStr) -> Result<Self> {
        let c_name = CString::new(name.as_encoded_bytes())?;
        let _guard = DATA_DIR_LOCK.lock().expect("no panic while locked");
        // SAFETY: `c_name` is NUL-terminated and outlives the call.
        let raw = unsafe { sys::dictionary_create_lang(c_name.as_ptr()) };
        let raw = NonNull::new(raw).ok_or_else(|| {
            Error::DictionaryNotFound(name.display().to_string())
        })?;
        let dictionary = Self {
            raw,
            language: PhantomData,
        };
        // A failure drops `dictionary`, which deletes it.
        dictionary.check_relations()?;
        Ok(dictionary)
    }

    /// Fails unless each link type number names the variant whose code is
    /// the link type. A stale enum fails here instead of mislabelling
    /// links.
    fn check_relations(&self) -> Result<()> {
        let stale = self.internals().connectors().any(|connector| {
            let name = connector.link_type_name();
            let expected =
                if name.starts_with("_I") { "_I" } else { name };
            L::Relation::from_number(connector.uc_num).code() != expected
        });
        if stale {
            return Err(Error::StaleRelations);
        }
        Ok(())
    }

    /// The handle the C API takes.
    pub(crate) fn as_raw(&self) -> sys::Dictionary { self.raw.as_ptr() }
}

impl<L: Language> Drop for Dictionary<L> {
    fn drop(&mut self) {
        // SAFETY: `raw` came from `dictionary_create_lang`, and every
        // sentence borrowing it is already gone.
        unsafe { sys::dictionary_delete(self.raw.as_ptr()) };
    }
}

/// Makes later [`Dictionary::new`] calls look in `path` first. `path`
/// holds one subdirectory per language.
pub fn set_data_dir(path: &Path) -> Result<()> {
    let c_path = CString::new(path.as_os_str().as_encoded_bytes())?;
    let _guard = DATA_DIR_LOCK.lock().expect("no panic while locked");
    // SAFETY: `c_path` is NUL-terminated and link-grammar copies it.
    unsafe { sys::dictionary_set_data_dir(c_path.as_ptr()) };
    Ok(())
}

/// Serializes access to link-grammar's global data directory, which
/// `dictionary_set_data_dir` writes and `dictionary_create_lang` reads.
static DATA_DIR_LOCK: Mutex<()> = Mutex::new(());
