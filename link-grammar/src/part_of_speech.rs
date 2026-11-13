/// The word class that an English dictionary entry's subscript names,
/// such as [`Verb`](Self::Verb) for `landed.v-d`. A suffix after `-`
/// narrows the class, as in `v-d` for a past-tense verb.
///
/// [`Word::part_of_speech`](crate::Word::part_of_speech) gives a word's
/// class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PartOfSpeech {
    /// `.n`, including mass nouns, `.n-u`. The narrower noun groups
    /// are `.s`, `.t` for titles, `.u` for units, `.l` for places, and
    /// `.m`, `.f` and `.b` for male, female and either-gender names and
    /// relatives.
    Noun,
    /// `.v`, including past tenses, `.v-d`. The narrower verb groups are
    /// `.q` for paraphrasing verbs like "say", and `.w` for verbs with a
    /// special sense.
    Verb,
    /// `.g`, a verb's `-ing` form used as a noun.
    Gerund,
    /// `.a`, including comparatives, `.a-c`, and superlatives, `.a-s`.
    Adjective,
    /// `.e`.
    Adverb,
    /// `.j`, with a suffix that says what it joins, as in `and.j-n`.
    Conjunction,
}

impl PartOfSpeech {
    /// The class an English `subscript` names, if it names a broad one.
    pub(crate) fn from_english_subscript(subscript: &str) -> Option<Self> {
        let class = subscript
            .split_once('-')
            .map_or(subscript, |(class, _)| class);
        match class {
            "n" | "s" | "t" | "u" | "l" | "m" | "f" | "b" => {
                Some(Self::Noun)
            }
            "v" | "q" | "w" => Some(Self::Verb),
            "g" => Some(Self::Gerund),
            "a" => Some(Self::Adjective),
            "e" => Some(Self::Adverb),
            "j" => Some(Self::Conjunction),
            _ => None,
        }
    }
}
