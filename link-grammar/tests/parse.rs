//! Parses with the bundled English dictionary and checks the result
//! against `link-parser en`.

use link_grammar::{
    Dictionary, Language, ParseOptions, PartOfSpeech, Relation, Sentence,
    WordGuess,
};

#[test]
fn parse_finds_link_parsers_linkages() -> link_grammar::Result<()> {
    let dictionary = Dictionary::new(Language::English)?;
    let mut sentence =
        Sentence::new(&dictionary, "The lander touched down on Mars.")?;

    assert_eq!(sentence.parse()?, 16);
    assert_eq!(sentence.linkages().count(), 16);
    assert_eq!(sentence.null_count(), 0);

    let first = sentence.linkage(0).expect("a first linkage");
    let labels: Vec<&str> =
        first.links().map(|link| link.label.as_str()).collect();
    for label in ["Wd", "D*u", "Ss", "K", "MVp", "Js", "Xp", "WV"] {
        assert!(
            labels.contains(&label),
            "{label} is missing from {labels:?}"
        );
    }
    Ok(())
}

#[test]
fn linkage_twice_returns_the_same_words() -> link_grammar::Result<()> {
    let dictionary = Dictionary::new(Language::English)?;
    let mut sentence = Sentence::new(&dictionary, "Mars is red.")?;
    sentence.parse()?;

    let first = sentence.linkage(0).expect("a first linkage");
    let again = sentence.linkage(0).expect("a first linkage");
    assert!(first.words().eq(again.words()));
    Ok(())
}

#[test]
fn labels_split_into_link_type_and_subscript() -> link_grammar::Result<()>
{
    let dictionary = Dictionary::new(Language::English)?;
    let mut sentence =
        Sentence::new(&dictionary, "The lander touched down on Mars.")?;
    sentence.parse()?;

    let first = sentence.linkage(0).expect("a first linkage");
    let parts: Vec<(&str, &str)> = first
        .links()
        .map(|link| {
            (
                link.label.link_type().as_str(),
                link.label.subscript().as_str(),
            )
        })
        .collect();
    for part in [("D", "*u"), ("MV", "p"), ("K", ""), ("WV", "")] {
        assert!(
            parts.contains(&part),
            "{part:?} is missing from {parts:?}"
        );
    }
    Ok(())
}

#[cfg(feature = "bundled")]
#[test]
fn from_path_loads_the_bundled_english() -> link_grammar::Result<()> {
    let path = std::path::Path::new(link_grammar_sys::BUNDLED_DATA_DIR)
        .join("en");
    let dictionary = Dictionary::from_path(&path)?;
    assert_eq!(dictionary.language(), Some(Language::English));
    let mut sentence = Sentence::new(&dictionary, "Mars is red.")?;
    assert!(sentence.parse()? > 0);
    Ok(())
}

#[test]
fn words_split_into_text_guess_and_subscript() -> link_grammar::Result<()>
{
    let dictionary = Dictionary::new(Language::English)?;
    let mut sentence = Sentence::new(
        &dictionary,
        "Mr. Smith bought 123 widgets in the U.S. yesterday.",
    )?;
    sentence.parse()?;

    let first = sentence.linkage(0).expect("a first linkage");
    let words: Vec<_> = first
        .words()
        .map(|word| (word.text(), word.guess(), word.subscript()))
        .collect();
    for word in [
        ("LEFT-WALL", None, None),
        ("Mr.", None, Some("x")),
        ("bought", None, Some("v-d")),
        ("123", Some(WordGuess::Regex), None),
        ("widgets", Some(WordGuess::Regex), Some("n")),
        ("U.S.", Some(WordGuess::Regex), None),
        ("the", None, None),
    ] {
        assert!(
            words.contains(&word),
            "{word:?} is missing from {words:?}"
        );
    }
    Ok(())
}

#[test]
fn null_parses_mark_unlinked_words() -> link_grammar::Result<()> {
    let dictionary = Dictionary::new(Language::English)?;
    let mut options = ParseOptions::new();
    options.set_max_null_count(1);
    let mut sentence = Sentence::new(
        &dictionary,
        "The the lander touched down on Mars.",
    )?;
    sentence.parse_with(&options)?;

    assert_eq!(sentence.null_count(), 1);
    let first = sentence.linkage(0).expect("a first linkage");
    let unlinked: Vec<_> = first
        .words()
        .filter(|word| word.is_unlinked())
        .map(|word| word.text())
        .collect();
    assert_eq!(unlinked, ["the"]);
    let lander = first
        .words()
        .find(|word| word.text() == "lander")
        .expect("lander is a word");
    assert_eq!(lander.guess(), Some(WordGuess::Unknown));
    assert_eq!(lander.subscript(), Some("n"));
    Ok(())
}

#[test]
fn english_words_and_links_have_types() -> link_grammar::Result<()> {
    let dictionary = Dictionary::new(Language::English)?;
    let mut sentence = Sentence::new(
        &dictionary,
        "The happy dogs quickly ate red apples and slept.",
    )?;
    sentence.parse()?;

    let first = sentence.linkage(0).expect("a first linkage");
    let words: Vec<_> = first
        .words()
        .map(|word| (word.text(), word.part_of_speech()))
        .collect();
    for word in [
        ("the", None),
        ("happy", Some(PartOfSpeech::Adjective)),
        ("dogs", Some(PartOfSpeech::Noun)),
        ("quickly", None),
        ("ate", Some(PartOfSpeech::Verb)),
        ("and", Some(PartOfSpeech::Conjunction)),
    ] {
        assert!(
            words.contains(&word),
            "{word:?} is missing from {words:?}"
        );
    }
    let relations: Vec<_> = first
        .links()
        .map(|link| link.label.link_type().relation())
        .collect();
    for relation in [
        Relation::Determiner,
        Relation::AttributiveAdjective,
        Relation::Subject,
        Relation::PreVerbAdverb,
        Relation::Object,
        Relation::VerbConjunction,
    ] {
        assert!(
            relations.contains(&Some(relation)),
            "{relation:?} is missing from {relations:?}"
        );
    }
    Ok(())
}
