//! Checks that the readers over link-grammar's private structs agree
//! with the C getters and the printed words, sentence by sentence of the
//! English corpus.

use std::fs;
use std::path::Path;

use link_grammar_sys as sys;

use crate::c_str_ext::CStrPtr;
use crate::{
    Dictionary, Language, Linkage, Relation, Sentence, Verbosity,
};

#[test]
fn readers_agree_with_the_c_getters() {
    crate::set_verbosity(Verbosity::Silent);
    let dictionary =
        Dictionary::new(Language::English).expect("English is bundled");
    for text in corpus() {
        let mut sentence = Sentence::new(&dictionary, &text)
            .expect("corpus text has no NUL");
        if sentence.parse().is_err() {
            continue;
        }
        for linkage in sentence.linkages() {
            assert_links_agree(&text, &linkage);
            assert_words_agree(&text, &linkage);
        }
    }
}

/// Every link's label, relation, subscript and ends, read both ways. A
/// wrong struct layout, say after the submodule tag moves, gives garbage
/// or undefined behaviour with no compile error, and this catches it.
fn assert_links_agree(text: &str, linkage: &Linkage<'_>) {
    for (index, link) in linkage.links().enumerate() {
        let label = c_label(linkage, index);
        assert_eq!(link.label().as_str(), label, "{link:?} in {text:?}");
        let name = link.label().link_type().as_str();
        assert_eq!(
            link.relation(),
            Relation::from_english_link_type(name),
            "relation of {link:?} in {text:?}",
        );
        assert_eq!(
            link.subscript(),
            link.label().subscript(),
            "subscript of {link:?} in {text:?}",
        );
        assert_eq!(
            (link.left, link.right),
            c_ends(linkage, index),
            "ends of {link:?} in {text:?}",
        );
    }
}

/// Every word's text against its printed form.
fn assert_words_agree(text: &str, linkage: &Linkage<'_>) {
    for word in linkage.words() {
        // `as_str` calls `linkage_get_word`.
        assert_eq!(
            word.text(),
            printed_text(word.as_str()),
            "{word:?} in {text:?}",
        );
    }
}

/// `linkage_get_link_label`.
fn c_label<'l>(linkage: &'l Linkage<'_>, index: usize) -> &'l str {
    // SAFETY: `index` is below the link count, and the label lives as long
    // as the linkage.
    unsafe {
        sys::linkage_get_link_label(linkage.as_raw(), index).as_utf8()
    }
}

/// `linkage_get_link_lword` and `linkage_get_link_rword`.
fn c_ends(linkage: &Linkage<'_>, index: usize) -> (usize, usize) {
    let raw = linkage.as_raw();
    // SAFETY: `index` is below the link count.
    unsafe {
        (
            sys::linkage_get_link_lword(raw, index),
            sys::linkage_get_link_rword(raw, index),
        )
    }
}

/// The word in a printed form, without brackets, guess mark or subscript.
///
/// `[the] -> the`, `lander[?].n -> lander`, `Mr..x -> Mr.`, `3.5 -> 3.5`
fn printed_text(printed: &str) -> &str {
    // An unlinked word prints in brackets.
    let printed = printed
        .strip_prefix('[')
        .and_then(|inner| inner.strip_suffix(']'))
        .filter(|inner| !inner.is_empty())
        .unwrap_or(printed);
    // A guess mark is a `[` after the first byte, followed by one of four
    // marks.
    let guess = printed.rfind('[').filter(|&open| {
        open > 0 && printed[open + 1..].starts_with(['!', '~', '&', '?'])
    });
    match guess {
        Some(open) => &printed[..open],
        None => {
            subscript_dot(printed).map_or(printed, |dot| &printed[..dot])
        }
    }
}

/// The `.` that starts a subscript, the last one unless it ends the word
/// or comes before a digit or a non-ASCII byte. That is the rule
/// `patch_subscript` in link-grammar applies to dictionary entries.
fn subscript_dot(printed: &str) -> Option<usize> {
    printed.rfind('.').filter(|&dot| {
        printed[dot + 1..]
            .bytes()
            .next()
            .is_some_and(|next| next.is_ascii() && !next.is_ascii_digit())
    })
}

/// The sentences of `corpus-basic.batch`, without comments, commands,
/// blank lines and the `*` that marks an ungrammatical sentence.
fn corpus() -> Vec<String> {
    let path = Path::new(link_grammar_sys::BUNDLED_DATA_DIR)
        .join("en/corpus-basic.batch");
    fs::read_to_string(path)
        .expect("the bundled data has the English corpus")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter(|line| !line.starts_with(['%', '!']))
        .map(|line| line.strip_prefix('*').unwrap_or(line).to_owned())
        .collect()
}
