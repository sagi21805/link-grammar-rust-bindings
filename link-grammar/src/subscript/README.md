# Adding a relation to the named subscripts

A relation such as `S` gets one struct, with a field per subscript
position that has a named letter. `subject.rs` is the example to copy.

## Steps

1. **Find the letters.** Write the file in step 2 with one placeholder
   variant per position enum, add the test in step 4 with an empty
   `UNNAMED_*` list, and run it. It prints each letter the English
   dictionary uses, per position, with one connector that has it.
2. **Name the letters.** Create `subscript/<relation>.rs`. A letter gets a
   variant only when a comment in `4.0.dict` explains it, and the
   variant's doc cites the connector, an example sentence and the line.
   Never name a letter from how it is used, because most are marks that
   block another connector. A position with no named letter has no field
   and no enum.
3. **Register it.** Add `mod <relation>;` and a `pub use` line at the top
   of `subscript.rs`.
4. **Cover it.** In `coverage_tests.rs`, add a `Names` list with one
   closure per position, an `UNNAMED_<RELATION>` list of the letters left
   unnamed, and a test that calls `assert_only_known_unnamed`. The test
   fails on a letter in neither list, so a dictionary update that adds a
   letter shows up there.
5. **Run the checks.**

   ```sh
   cargo +nightly fmt --all
   cargo test -p link-grammar
   cargo clippy -p link-grammar --all-targets -- -D warnings
   cargo check -p link-grammar --no-default-features
   ```

6. **Commit.** One commit per relation, as `Add named subscript letters
   for the <relation> links.`

## When a letter has no comment

Ask before naming it. Put it in the test's `UNNAMED_*` list. It reads
`None` in the struct, and `Link::subscript` still has the raw letter.

## Template

The file for a relation `X` with a letter at position 0 and one at
position 2.

````rust
//! The named letters of an `X` link.

use super::{Subscript, SubscriptLetter};
use crate::{Relation, RelationSubscript};

/// The subscript of an `X` link, such as `Xs*e`.
///
/// ```
/// // A doctest that parses a sentence with an `X` link and checks one
/// // named letter.
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Xlink {
    /// Position 0.
    pub kind: Option<XlinkKind>,
    /// Position 2.
    pub usage: Option<XlinkUse>,
}

impl RelationSubscript for Xlink {
    const RELATION: Relation = Relation::X;

    fn from_subscript(subscript: Subscript) -> Self {
        Self {
            kind: XlinkKind::from_letter(subscript.letter(0)),
            usage: XlinkUse::from_letter(subscript.letter(2)),
        }
    }
}

subscript_letters! {
    /// Position 0 of an `X` link.
    pub enum XlinkKind {
        /// `Xs`, what it means. "An example sentence."
        Singular = b's',
    }
}
````
