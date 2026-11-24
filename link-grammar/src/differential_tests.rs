//! Checks that the readers over link-grammar's private structs agree
//! with the readers over its printed forms, sentence by sentence of the
//! English corpus.

use std::fs;
use std::path::Path;

use crate::{Dictionary, Language, Linkage, Sentence, Verbosity, Word};

#[test]
fn numeric_readers_agree_with_the_printed_forms() {
    assert_corpus_agrees(200);
}

#[test]
#[ignore = "parses all 934 sentences, which takes half a minute"]
fn numeric_readers_agree_with_the_printed_forms_on_the_whole_corpus() {
    assert_corpus_agrees(usize::MAX);
}

/// Parses the first `count` sentences of the corpus and checks every
/// linkage of each.
fn assert_corpus_agrees(count: usize) {
    crate::set_verbosity(Verbosity::Silent);
    let dictionary =
        Dictionary::new(Language::English).expect("English is bundled");
    for text in corpus().iter().take(count) {
        let mut sentence = Sentence::new(&dictionary, text)
            .expect("corpus text has no NUL");
        if sentence.parse().is_err() {
            continue;
        }
        for linkage in sentence.linkages() {
            assert_links_agree(text, &linkage);
            linkage
                .words()
                .for_each(|word| assert_word_agrees(text, word));
        }
    }
}

/// Every link's relation, subscript and equality, read both ways.
fn assert_links_agree(text: &str, linkage: &Linkage<'_>) {
    for link in linkage.links() {
        assert_eq!(
            link.relation(),
            link.relation_from_label(),
            "relation of {link:?} in {text:?}",
        );
        assert_eq!(
            link.subscript(),
            link.label().subscript(),
            "subscript of {link:?} in {text:?}",
        );
        for other in linkage.links() {
            let by_label = (link.left, link.right, link.label())
                == (other.left, other.right, other.label());
            assert_eq!(
                link == other,
                by_label,
                "equality of {link:?} and {other:?} in {text:?}",
            );
        }
    }
}

/// The word's text, guess, subscript and unlinked state, read both ways.
/// An unlinked wall prints without brackets, so only the entry says it's
/// unlinked, and the printed form reads it as linked.
fn assert_word_agrees(text: &str, word: Word<'_>) {
    let is_unlinked_wall =
        word.is_unlinked() && word.unlinked_inner().is_none();
    if !is_unlinked_wall {
        assert_eq!(
            word.is_unlinked(),
            word.unlinked_inner().is_some(),
            "unlinked state of {word:?} in {text:?}",
        );
    }
    let parts = word.parts();
    assert_eq!(
        (word.text(), word.guess(), word.subscript()),
        (parts.text, parts.guess, parts.subscript),
        "{word:?} in {text:?}",
    );
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
