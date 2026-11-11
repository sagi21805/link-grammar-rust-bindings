use std::ffi::CString;
use std::path::Path;
use std::ptr::NonNull;
use std::sync::Mutex;

use link_grammar_sys as sys;

use crate::{Error, Result};

/// The grammar and word list of one language.
///
/// [`Sentence`](crate::Sentence)s borrow the dictionary they are parsed
/// with, so it outlives all of them.
///
/// ```no_run
/// use std::path::Path;
///
/// use link_grammar::Dictionary;
///
/// Dictionary::set_data_dir(Path::new("link-grammar/data"))?;
/// let english = Dictionary::new("en")?;
/// # Ok::<(), link_grammar::Error>(())
/// ```
// TODO: Upstream allows sharing one dictionary between threads. Add
// `Send` and `Sync` once something needs it.
pub struct Dictionary {
    raw: NonNull<sys::Dictionary_s>,
}

impl Dictionary {
    /// Loads the dictionary for `lang` (such as `"en"` or `"ru"`), or one
    /// at the path `lang` names.
    ///
    /// link-grammar searches the directory given to
    /// [`set_data_dir`](Self::set_data_dir), then its install directory,
    /// then `./data`.
    pub fn new(lang: &str) -> Result<Self> {
        let c_lang = CString::new(lang)?;
        let _guard = DATA_DIR_LOCK.lock().expect("no panic while locked");
        // SAFETY: `c_lang` is NUL-terminated and outlives the call.
        let raw = unsafe { sys::dictionary_create_lang(c_lang.as_ptr()) };
        NonNull::new(raw)
            .map(|raw| Self { raw })
            .ok_or_else(|| Error::DictionaryNotFound(lang.to_owned()))
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
