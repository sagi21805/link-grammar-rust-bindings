//! Prints the link types of one language's dictionary, one per line, in
//! link type number order. `cargo xtask relations` reads the output.
//!
//! ```sh
//! cargo run -p link-grammar-sys --example link_types -- en
//! ```

use std::collections::BTreeMap;
use std::ffi::{CStr, CString};
use std::{env, slice};

use link_grammar_sys::{self as sys, internals};

fn main() {
    let code = env::args().nth(1).expect("usage: link_types <language>");
    let c_code = CString::new(code.as_str()).expect("a code has no NUL");
    // SAFETY: `c_code` is NUL-terminated and outlives the call.
    let raw = unsafe { sys::dictionary_create_lang(c_code.as_ptr()) };
    assert!(!raw.is_null(), "no dictionary for `{code}`");
    // SAFETY: `raw` came from `dictionary_create_lang`, so it points to a
    // full `Dictionary_s`.
    let dictionary = unsafe { &*raw.cast::<internals::Dictionary_s>() };
    for name in link_types(dictionary) {
        println!("{name}");
    }
    // SAFETY: `raw` came from `dictionary_create_lang`, and nothing reads
    // it after this.
    unsafe { sys::dictionary_delete(raw) };
}

/// The names of the dictionary's link types other than idioms, in number
/// order. Panics unless the numbers run 0, 1, 2, .. with no gap.
fn link_types(dictionary: &internals::Dictionary_s) -> Vec<String> {
    let mut names = BTreeMap::new();
    for connector in connectors(dictionary) {
        let name = link_type_name(connector);
        // Idioms share one variant, which `link_relations!` adds.
        if name.starts_with("_I") {
            continue;
        }
        // Connectors such as `Ss` and `Sp` share the number of `S`.
        let known = names.entry(connector.uc_num).or_insert(name);
        assert_eq!(*known, name, "one number, two link types");
    }
    assert!(
        names.keys().copied().eq(0..names.len() as u32),
        "link type numbers have a gap",
    );
    names.into_values().map(str::to_owned).collect()
}

/// Every connector descriptor the dictionary has.
fn connectors(
    dictionary: &internals::Dictionary_s,
) -> impl Iterator<Item = &internals::condesc_struct> {
    let table = &dictionary.contable;
    // SAFETY: `hdesc` has `size` slots, each NULL or a descriptor that
    // lives as long as the dictionary.
    let slots = unsafe { slice::from_raw_parts(table.hdesc, table.size) };
    // SAFETY: as above.
    slots
        .iter()
        .filter_map(|slot| unsafe { slot.desc.as_ref() })
}

/// The connector's uppercase part, such as `MV` in `MVp`.
fn link_type_name(connector: &internals::condesc_struct) -> &str {
    // SAFETY: `more` and its `string` live as long as the dictionary.
    let more = unsafe { &*connector.more };
    let start = usize::from(more.uc_start);
    // SAFETY: as above, and the string is NUL-terminated.
    let name = unsafe { CStr::from_ptr(more.string) };
    let name = name.to_str().expect("connector names are ASCII");
    &name[start..start + usize::from(more.uc_length)]
}
