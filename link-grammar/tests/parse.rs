//! Parses with the bundled English dictionary and checks the result
//! against `link-parser en`.

use link_grammar::{Dictionary, Language, ParseOptions, Sentence};

#[test]
fn parse_finds_link_parsers_linkages() -> link_grammar::Result<()> {
    let dictionary = Dictionary::new(Language::English)?;
    let options = ParseOptions::new();
    let mut sentence =
        Sentence::new(&dictionary, "The lander touched down on Mars.")?;

    assert_eq!(sentence.parse(&options)?, 16);
    assert_eq!(sentence.linkages(&options).count(), 16);
    assert_eq!(sentence.null_count(), 0);

    let first = sentence.linkage(0, &options).expect("a first linkage");
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
    let options = ParseOptions::new();
    let mut sentence = Sentence::new(&dictionary, "Mars is red.")?;
    sentence.parse(&options)?;

    let first = sentence.linkage(0, &options).expect("a first linkage");
    let again = sentence.linkage(0, &options).expect("a first linkage");
    assert!(first.words().eq(again.words()));
    Ok(())
}

#[test]
fn labels_split_into_link_type_and_subscript() -> link_grammar::Result<()>
{
    let dictionary = Dictionary::new(Language::English)?;
    let options = ParseOptions::new();
    let mut sentence =
        Sentence::new(&dictionary, "The lander touched down on Mars.")?;
    sentence.parse(&options)?;

    let first = sentence.linkage(0, &options).expect("a first linkage");
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
    let options = ParseOptions::new();
    let mut sentence = Sentence::new(&dictionary, "Mars is red.")?;
    assert!(sentence.parse(&options)? > 0);
    Ok(())
}
