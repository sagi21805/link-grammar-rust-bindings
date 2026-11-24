//! The cost of reading and matching the links and words of parsed
//! linkages, with the parse itself left out.

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use link_grammar::{
    Dictionary, Language, Link, Linkage, Relation, Sentence, Verbosity,
};

/// Sentences from `corpus-basic.batch`, which parse into about a
/// thousand linkages between them.
const SENTENCES: &[&str] = &[
    "The fact that he smiled at me gives me hope",
    "I read everything I could about the subject",
    "Almost three years after our first date, I saw Ruth again",
    "How important is it to turn the computer off",
    "The public seem to love him, no matter what he does",
    "I told Margaret that I thought she would probably be hired",
    "Using specially designed parts, Baldwin pianos are technical marvels",
    "I'm sure things will get done with Janet running the company",
    "I may have taken cocaine a few times, but at no time did I inhale",
    "This seems to have been because he is extremely famous",
];

fn links(c: &mut Criterion) {
    link_grammar::set_verbosity(Verbosity::Silent);
    let dictionary =
        Dictionary::new(Language::English).expect("English is bundled");
    let sentences = parsed(&dictionary);
    let linkages: Vec<Linkage<'_>> = sentences
        .iter()
        .flat_map(|sentence| sentence.linkages())
        .collect();

    c.bench_function("relation", |b| {
        b.iter(|| {
            every_link(&linkages)
                .filter(|link| link.relation().is_some())
                .count()
        })
    });
    c.bench_function("singular subject", |b| {
        b.iter(|| {
            every_link(&linkages)
                .filter(|link| is_singular_subject(link))
                .count()
        })
    });
    c.bench_function("word subscript and guess", |b| {
        b.iter(|| {
            linkages
                .iter()
                .flat_map(|linkage| linkage.words())
                .for_each(|word| {
                    black_box((word.subscript(), word.guess()));
                })
        })
    });
}

/// The links of every linkage in `linkages`.
fn every_link<'l>(
    linkages: &'l [Linkage<'l>],
) -> impl Iterator<Item = Link<'l>> {
    linkages.iter().flat_map(|linkage| linkage.links())
}

/// Whether `link` joins a singular subject to its verb, as `Ss` does.
fn is_singular_subject(link: &Link<'_>) -> bool {
    link.relation() == Some(Relation::Subject)
        && link.label().subscript().as_str().starts_with('s')
}

/// Each of [`SENTENCES`], parsed once.
fn parsed(dictionary: &Dictionary) -> Vec<Sentence<'_>> {
    SENTENCES
        .iter()
        .map(|text| {
            let mut sentence = Sentence::new(dictionary, text)
                .expect("the sentences have no NUL");
            sentence.parse().expect("the sentences parse");
            sentence
        })
        .collect()
}

criterion_group!(benches, links);
criterion_main!(benches);
