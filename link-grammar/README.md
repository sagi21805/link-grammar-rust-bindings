# link-grammar

Safe Rust bindings to [link-grammar](https://github.com/opencog/link-grammar),
the CMU/OpenCog parser that links the words of a sentence with labelled
grammatical relations.

```rust
use link_grammar::english::{English, Relation};
use link_grammar::{Dictionary, Sentence};

fn main() -> link_grammar::Result<()> {
    let dictionary = Dictionary::new(English)?;
    let mut sentence = Sentence::new(&dictionary, "The lander landed.")?;
    sentence.parse()?;
    let linkage = sentence.linkage(0).expect("a parse");
    for link in linkage.links() {
        if link.label().relation == Relation::S {
            println!("{} is the subject", link.left_word().text());
        }
    }
    Ok(())
}
```

`Linkage::diagram` draws a linkage the way link-parser does. This is the
diagram for "The lander touched down on Mars."

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

Only English ships with the crate. Point `link_grammar::set_data_dir` at
link-grammar's `data/` to load another language's dictionary, once the
crate has a marker type for it.

## License

LGPL-2.1, the same as link-grammar. The build links link-grammar statically,
so read the LGPL's section 6 before you distribute a binary.
