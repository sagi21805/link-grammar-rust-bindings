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

The build compiles link-grammar 5.13.2 from the sources that
`link-grammar-sys` ships, along with the English dictionary, so you need
only a C compiler. It supports Linux with glibc and macOS for now.

Only English ships with the crate. Load the others with
`Dictionary::set_data_dir`.

## License

LGPL-2.1, the same as link-grammar. The build links link-grammar statically,
so read the LGPL's section 6 before you distribute a binary.
