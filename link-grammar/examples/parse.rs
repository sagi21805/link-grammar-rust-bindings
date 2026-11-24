//! Prints the best linkage of each line on stdin as a diagram.
//!
//! ```sh
//! echo "The lander touched down on Mars." | cargo run --example parse
//! ```

use std::io::{self, BufRead};

use link_grammar::{
    Dictionary, Language, Linkage, Sentence, Verbosity, Word,
    subscript::{Subject, SubjectKind},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    link_grammar::set_verbosity(Verbosity::Silent);
    let dictionary = Dictionary::new(Language::English)?;
    for line in io::stdin().lock().lines().take(1) {
        let mut sentence = Sentence::new(&dictionary, &line?)?;
        let found = sentence.parse()?;
        println!("Found {found} linkages");
        if let Some(linkage) = sentence.linkage(0) {
            println!("{}", linkage.diagram(80));

            let singular_subjects = singluar_subjects(&linkage)
                .iter()
                .map(|w| w.text())
                .collect::<Vec<_>>();

            println!("{singular_subjects:#?}");
        }
    }
    Ok(())
}

fn singluar_subjects<'a>(l: &'a Linkage) -> Vec<Word<'a>> {
    l.links_as::<Subject>()
        .filter_map(|(l, s)| {
            if let Some(kind) = s.kind
                && let SubjectKind::Singular = kind
            {
                Some(l.left_word())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}
