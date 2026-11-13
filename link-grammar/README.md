# link-grammar

Safe Rust bindings to [link-grammar](https://github.com/opencog/link-grammar),
the CMU/OpenCog parser that links the words of a sentence with labelled
grammatical relations.

```rust
use link_grammar::{Dictionary, Language, Sentence};

fn main() -> link_grammar::Result<()> {
    let dictionary = Dictionary::new(Language::English)?;
    let mut sentence =
        Sentence::new(&dictionary, "The lander touched down on Mars.")?;
    sentence.parse()?;
    if let Some(linkage) = sentence.linkage(0) {
        println!("{}", linkage.diagram(80));
    }
    Ok(())
}
```

```text
    +------------------------Xp------------------------+
    +----------->WV----------->+                       |
    +----->Wd------+           +------MVp-----+        |
    |      +--D*u--+-----Ss----+----K----+    +-Js-+   |
    |      |       |           |         |    |    |   |
LEFT-WALL the lander[?].n touched.v-d down.r on Mars.m .
```

## Building

The default `bundled` feature compiles link-grammar 5.13.2 from the
sources that `link-grammar-sys` ships, along with the English dictionary,
so you need only a C compiler. It supports Linux with glibc and macOS for now.

To link a link-grammar you installed yourself, which pkg-config must find,
turn off `bundled`:

```toml
[dependencies]
link-grammar = { version = "0.1", default-features = false }
```

`Language` lists the languages link-grammar has dictionaries for. Only
English ships with the crate, so point `Dictionary::set_data_dir` at
link-grammar's `data/` directory for the others, or load any dictionary
directory with `Dictionary::from_path`.

Each `Link` carries a `LinkLabel`, such as `MVp`, which splits into a
`LinkType` (`MV`) and a `Subscript` (`p`). They borrow the linkage's
string, so only a parse can make one. For English, `LinkType::relation`
names the link's grammatical `Relation`, such as `Relation::Subject` for
`S`, and `Word::part_of_speech` reads a word's `PartOfSpeech` from its
subscript.

## License

LGPL-2.1, the same as link-grammar. The `bundled` build links link-grammar
statically, so read the LGPL's section 6 before you distribute a binary.
