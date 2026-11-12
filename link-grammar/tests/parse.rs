//! Parses with the bundled English dictionary and checks the result
//! against `link-parser en`.

use link_grammar::{Dictionary, ParseOptions, Sentence};

#[test]
fn parse_finds_link_parsers_linkages() -> link_grammar::Result<()> {
    let dictionary = Dictionary::new("en")?;
    let options = ParseOptions::new();
    let mut sentence =
        Sentence::new(&dictionary, "The lander touched down on Mars.")?;

    assert_eq!(sentence.parse(&options)?, 16);
    assert_eq!(sentence.linkages(&options).count(), 16);
    assert_eq!(sentence.null_count(), 0);

    let first = sentence.linkage(0, &options).expect("a first linkage");
    let labels: Vec<&str> = first.links().map(|link| link.label).collect();
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
    let dictionary = Dictionary::new("en")?;
    let options = ParseOptions::new();
    let mut sentence = Sentence::new(&dictionary, "Mars is red.")?;
    sentence.parse(&options)?;

    let first = sentence.linkage(0, &options).expect("a first linkage");
    let again = sentence.linkage(0, &options).expect("a first linkage");
    assert!(first.words().eq(again.words()));
    Ok(())
}
