relations! {
    /// The grammatical relation that an English [`LinkType`] names, such
    /// as [`Subject`](Self::Subject) for `S`.
    ///
    /// [`LinkType::relation`] gives a link's relation. Each variant's doc
    /// starts with its link type, and the [English Link Documentation]
    /// describes each one with examples.
    ///
    /// [`LinkType`]: crate::LinkType
    /// [`LinkType::relation`]: crate::LinkType::relation
    /// [English Link Documentation]: https://opencog.github.io/link-grammar-website/dict/index.html
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum Relation {
        /// `A` connects adjectives to the nouns they come before.
        "A" => AttributiveAdjective,
        /// `AA` connects the adjective of "how big a dog" to the "a".
        "AA" => HowAdjectiveA,
        /// `AF` connects an adjective fronted in a question to its verb.
        "AF" => FrontedAdjective,
        /// `AJ` connects adjectives to the conjunction that joins them.
        "AJ" => AdjectiveConjunction,
        /// `AL` connects "all" and "both" to the determiner after them.
        "AL" => Predeterminer,
        /// `AM` connects "as" to "much" or "many".
        "AM" => AsMuch,
        /// `AN` connects a noun used as a modifier to the noun after it,
        /// as in "tax proposal".
        "AN" => ModifyingNoun,
        /// `AZ` connects verbs like "imagine" to the "as" of their
        /// complement, as in "imagined Mary as innocent".
        "AZ" => AsComplement,
        /// `B` connects transitive verbs back to objects that come before
        /// them in relative clauses and questions.
        "B" => FrontedObject,
        /// `BI` connects forms of "be" to some idioms.
        "BI" => BeIdiom,
        /// `BW` connects "what" to verbs like "think" in questions.
        "BW" => WhatVerb,
        /// `C` connects conjunctions and some verbs to the subjects of the
        /// clauses they take.
        "C" => ClauseSubject,
        /// `CO` connects openers to the subject of their clause.
        "CO" => Opener,
        /// `CP` connects paraphrasing and quoting verbs to the wall.
        "CP" => Paraphrase,
        /// `CQ` connects to auxiliaries in comparatives with subject-verb
        /// inversion.
        "CQ" => InvertedComparative,
        /// `CV` connects the verb of a subordinate clause to the word that
        /// starts it.
        "CV" => SubordinateVerb,
        /// `CX` connects comparatives whose second half is only an
        /// auxiliary.
        "CX" => ComparativeAuxiliary,
        /// `D` connects determiners to nouns.
        "D" => Determiner,
        /// `DD` connects definite determiners to numbers and to adjectives
        /// used as nouns.
        "DD" => DefiniteDeterminer,
        /// `DG` connects "the" to proper nouns.
        "DG" => TheProperNoun,
        /// `DP` connects possessive determiners to gerunds.
        "DP" => PossessiveGerund,
        /// `DT` connects determiners to nouns in time idioms, such as
        /// "next week".
        "DT" => TimeDeterminer,
        /// `E` connects adverbs to the verbs after them.
        "E" => PreVerbAdverb,
        /// `EA` connects adverbs to adjectives.
        "EA" => AdjectiveAdverb,
        /// `EB` connects adverbs to forms of "be" before an object or a
        /// prepositional phrase.
        "EB" => BeAdverb,
        /// `EC` connects adverbs to comparative adjectives.
        "EC" => ComparativeAdverb,
        /// `EE` connects adverbs to other adverbs.
        "EE" => AdverbAdverb,
        /// `EF` connects "enough" to the adjective or adverb before it.
        "EF" => Enough,
        /// `EI` connects an opener set off by a comma to the imperative
        /// after it, as in "Hey, stop that!".
        "EI" => ImperativeOpener,
        /// `EL` connects words like "something" and "someone" to "else".
        "EL" => Else,
        /// `EN` connects some adverbs to quantities.
        "EN" => QuantityAdverb,
        /// `EP` connects "just" and "maybe" to prepositions of place.
        "EP" => PrepositionAdverb,
        /// `EQ` connects the parts of an equation.
        "EQ" => Equation,
        /// `ER` connects the halves of "the more ..., the merrier".
        "ER" => DoubleComparative,
        /// `ET` connects adverbs like "soon" and "shortly" to "after" and
        /// "before".
        "ET" => AfterBeforeAdverb,
        /// `EW` connects some adverbs to question words in direct
        /// questions.
        "EW" => QuestionWordAdverb,
        /// `EZ` connects adverbs like "just" and "almost" to "as".
        "EZ" => AsAdverb,
        /// `FL` connects "for" to "long".
        "FL" => ForLong,
        /// `FM` connects "from" to other prepositions.
        "FM" => FromPreposition,
        /// `G` connects proper nouns in a series, as in "Harry Potter".
        "G" => ProperNounSeries,
        /// `GN` connects a proper noun to the common noun before it that
        /// introduces it.
        "GN" => IntroducedProperNoun,
        /// `H` connects "how" to "much" or "many".
        "H" => HowMuch,
        /// `HA` connects "how" to "a".
        "HA" => HowA,
        /// `HM` connects hesitation markers, such as "uh".
        "HM" => Hesitation,
        /// `_I` connects the words of an idiom. link-grammar generates a
        /// link type for each idiom, such as `_IB`.
        "_I" => Idiom,
        /// `I` connects infinitives to modals and to "to".
        "I" => Infinitive,
        /// `IN` connects "in" to time expressions.
        "IN" => InTime,
        /// `IV` connects an infinitive to the clause it completes.
        "IV" => InfinitiveClause,
        /// `J` connects prepositions to their objects.
        "J" => PrepositionObject,
        /// `JG` connects some prepositions to proper nouns.
        "JG" => PrepositionProperNoun,
        /// `JQ` connects prepositions to question-word determiners.
        "JQ" => PrepositionQuestionWord,
        /// `JT` connects some conjunctions to time expressions like "last
        /// week".
        "JT" => ConjunctionTime,
        /// `K` connects verbs to particles like "in", "out" and "up".
        "K" => Particle,
        /// `L` connects some determiners to superlatives.
        "L" => Superlative,
        /// `LE` connects the adjective of a comparative to its second
        /// half past a complement.
        "LE" => ComparativeComplement,
        /// `LI` connects some verbs to "like".
        "LI" => Like,
        /// `M` connects nouns to the modifiers after them, such as
        /// prepositional phrases and participles.
        "M" => NounModifier,
        /// `MF` connects idioms with "some of them" after a noun phrase.
        "MF" => SomeOfThem,
        /// `MG` connects proper nouns to some prepositions that modify
        /// them.
        "MG" => ProperNounPreposition,
        /// `MJ` connects prepositions and other modifiers to the
        /// conjunction that joins them.
        "MJ" => ModifierConjunction,
        /// `MV` connects verbs and adjectives to the modifiers after them,
        /// such as adverbs and prepositional phrases.
        "MV" => VerbModifier,
        /// `MX` connects modifiers set off by commas to the noun before
        /// them.
        "MX" => CommaModifier,
        /// `N` connects "not" to the auxiliary before it.
        "N" => Not,
        /// `NA` connects the numbers of a spelled-out date.
        "NA" => DateNumber,
        /// `ND` connects numbers to expressions that take a number as
        /// their determiner.
        "ND" => NumberDeterminer,
        /// `NF` connects fractions to "of", as in "a quarter of an hour".
        "NF" => FractionOf,
        /// `NJ` connects fractions to the unit after them, as in "half an
        /// hour".
        "NJ" => FractionUnit,
        /// `NI` connects a few idiomatic number phrases.
        "NI" => NumberIdiom,
        /// `NM` connects numbers to the noun they label, as in "a No. 2
        /// pencil".
        "NM" => NumericalModifier,
        /// `NN` connects number words in a series.
        "NN" => NumberSeries,
        /// `NR` connects fractions to superlatives.
        "NR" => FractionSuperlative,
        /// `NS` connects "one", "1" and "a" to expressions that need a
        /// number.
        "NS" => SingularNumber,
        /// `NT` connects "not" to "to".
        "NT" => NotTo,
        /// `NW` connects idiomatic fractions, such as "two thirds".
        "NW" => FractionIdiom,
        /// `O` connects transitive verbs to their direct or indirect
        /// objects.
        "O" => Object,
        /// `OD` connects verbs like "rise" and "fall" to distances.
        "OD" => Distance,
        /// `OF` connects some verbs and adjectives to "of".
        "OF" => Of,
        /// `OH` connects vocatives to proper nouns.
        "OH" => Vocative,
        /// `ON` connects "on" to dates and days of the week.
        "ON" => OnDate,
        /// `OX` connects verbs to "it" and "there" used as objects.
        "OX" => FillerObject,
        /// `P` connects forms of "be" to prepositions, adjectives and
        /// participles.
        "P" => BeComplement,
        /// `PF` connects the verb to its subject when a preposition starts
        /// the sentence.
        "PF" => FrontedPreposition,
        /// `PH` makes "a" and "an" agree with the sound of the next word.
        "PH" => Phonetic,
        /// `PP` connects forms of "have" to past participles.
        "PP" => PastParticiple,
        /// `Q` connects the wall or a question word to the auxiliary of a
        /// question.
        "Q" => Question,
        /// `QI` connects some verbs and adjectives to question words, as
        /// in "I wonder why".
        "QI" => IndirectQuestion,
        /// `QJ` connects question words to the conjunction that joins
        /// them.
        "QJ" => QuestionConjunction,
        /// `QN` connects most verbs to question words in inverted
        /// questions.
        "QN" => InvertedQuestion,
        /// `QU` connects quotation marks to quoted sentences.
        "QU" => Quotation,
        /// `R` connects nouns to their relative clauses.
        "R" => RelativeClause,
        /// `RJ` connects adverbs and other phrases to the conjunction that
        /// joins them.
        "RJ" => PhraseConjunction,
        /// `RS` connects the relative pronoun of a subject relative clause
        /// to its verb.
        "RS" => RelativeSubject,
        /// `RW` connects the right wall to the left wall when no
        /// punctuation needs it.
        "RW" => RightWall,
        /// `S` connects subject nouns to finite verbs.
        "S" => Subject,
        /// `SF` connects "it" and "there" used as subjects to finite
        /// verbs.
        "SF" => FillerSubject,
        /// `SFI` is [`FillerSubject`](Self::FillerSubject) with
        /// subject-verb inversion.
        "SFI" => InvertedFillerSubject,
        /// `SI` is [`Subject`](Self::Subject) with subject-verb inversion.
        "SI" => InvertedSubject,
        /// `SJ` connects nouns to the conjunction that joins them.
        "SJ" => NounConjunction,
        /// `SX` connects "I" to first-person verbs like "am" and "was".
        "SX" => FirstPersonSubject,
        /// `SXI` is [`FirstPersonSubject`](Self::FirstPersonSubject) with
        /// subject-verb inversion.
        "SXI" => InvertedFirstPersonSubject,
        /// `TA` connects adjectives like "late" to month names.
        "TA" => AdjectiveMonth,
        /// `TD` connects days of the week to times of day, as in "Monday
        /// morning".
        "TD" => DayTime,
        /// `TH` connects words that take a "that" clause to the "that".
        "TH" => That,
        /// `TI` connects titles like "president" that can go without a
        /// determiner.
        "TI" => Title,
        /// `TM` connects month names to day numbers.
        "TM" => MonthDay,
        /// `TO` connects verbs and adjectives that take an infinitive to
        /// "to".
        "TO" => To,
        /// `TR` connects determiners to comparatives.
        "TR" => DeterminerComparative,
        /// `TS` connects verbs like "suggest" that take a subjunctive
        /// clause to "that".
        "TS" => SubjunctiveThat,
        /// `TT` connects "time" to an adjective like "first" before it.
        "TT" => FirstTime,
        /// `TW` connects days of the week to dates.
        "TW" => DayDate,
        /// `TY` connects day numbers to year numbers.
        "TY" => Year,
        /// `TZ` connects times to time zones.
        "TZ" => TimeZone,
        /// `U` connects nouns in idioms like "kind of".
        "U" => KindOf,
        /// `UN` connects "until" and "since" to time phrases.
        "UN" => UntilSince,
        /// `V` connects verbs to the other words of their idioms.
        "V" => VerbIdiom,
        /// `VC` connects clauses to the conjunction after them.
        "VC" => ClauseConjunction,
        /// `VJ` connects verbs to the conjunction that joins them.
        "VJ" => VerbConjunction,
        /// `W` connects the left wall to the subject of the main clause,
        /// and conjunctions to the clauses after them.
        "W" => Wall,
        /// `WN` connects "when" to time nouns like "year".
        "WN" => WhenTime,
        /// `WR` connects "where" to a few verbs like "put" in questions.
        "WR" => WhereVerb,
        /// `WV` connects the left wall to the verb of the main clause.
        "WV" => VerbWall,
        /// `WY` connects "reason" to "why".
        "WY" => ReasonWhy,
        /// `X` connects punctuation to words and to other punctuation.
        "X" => Punctuation,
        /// `XJ` connects the halves of correlative conjunctions, such as
        /// "not only ... but".
        "XJ" => CorrelativeConjunction,
        /// `Y` connects quantities to the head of time and place idioms.
        "Y" => QuantityIdiom,
        /// `YP` connects plural nouns ending in "s" to a possessive
        /// apostrophe.
        "YP" => PluralPossessive,
        /// `YS` connects nouns to the possessive "'s".
        "YS" => Possessive,
        /// `Z` connects "as" to some verbs.
        "Z" => AsVerb,
        /// `ZZZ` connects the blanks that splitting words leaves behind.
        "ZZZ" => Blank,
    }
}

impl Relation {
    /// The relation an English `link_type` names, if it is one of the
    /// dictionary's.
    pub(crate) fn from_english_link_type(link_type: &str) -> Option<Self> {
        match link_type {
            idiom if idiom.starts_with("_I") => Some(Self::Idiom),
            link_type => Self::from_table(link_type),
        }
    }
}
