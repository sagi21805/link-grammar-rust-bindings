//! The lowercase part of a link label, packed into a number, and the
//! structs that name its letters.
//!
//! To name the letters of another relation, follow `subscript/README.md`.

use std::fmt::{self, Write};

pub use object::{Object, ObjectFlag, ObjectKind, ObjectUse};
pub use subject::{Subject, SubjectConstraint, SubjectFlag, SubjectKind};

mod object;
mod subject;

/// The lowercase part of a link label, such as `p` in `MVp`, with up to 9
/// positions. Each position holds a letter or a `*`, which matches any
/// letter.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Subscript(u64);

/// The letters one subscript position takes, named.
pub(crate) trait SubscriptLetter: Sized {
    /// The variant for `letter`, which is 0 for a `*`.
    fn from_letter(letter: u8) -> Option<Self>;
}

/// The bits each position takes.
const LETTER_BITS: usize = 7;

/// The bits of position 0.
const LETTER_MASK: u64 = 0x7f;

/// The most positions a subscript has.
const POSITIONS: usize = 9;

impl Subscript {
    /// No subscript, as in a bare `S`. It matches every subscript.
    pub const EMPTY: Self = Self(0);

    /// Packs `subscript`, such as `"s*s"`. In a `const` item a bad
    /// subscript fails the build.
    ///
    /// # Panics
    ///
    /// When `subscript` has more than 9 positions or a byte outside `a-z`,
    /// `0-9` and `*`.
    pub const fn new(subscript: &str) -> Self {
        let bytes = subscript.as_bytes();
        assert!(bytes.len() <= POSITIONS, "a subscript has 9 positions");
        let mut position = 0;
        while position < bytes.len() {
            assert!(
                matches!(bytes[position], b'a'..=b'z' | b'0'..=b'9' | b'*'),
                "a subscript holds a-z, 0-9 and *",
            );
            position += 1;
        }
        Self::from_label(subscript)
    }

    /// The letter at `position`, or `None` for a `*` or a position past
    /// the end.
    pub const fn get(self, position: usize) -> Option<char> {
        match self.letter(position) {
            0 => None,
            letter => Some(letter as char),
        }
    }

    /// Whether link-grammar would connect the two, because they agree at
    /// every position where both have a letter.
    pub const fn matches(self, other: Self) -> bool {
        (self.0 ^ other.0) & self.letter_mask() & other.letter_mask() == 0
    }

    /// link-grammar's `lc_letters` without its head bit. Each position
    /// takes 7 bits holding the letter's ASCII value, position 0 lowest,
    /// and a `*` or a position past the end is 0.
    pub const fn to_bits(self) -> u64 { self.0 }

    /// Packs a subscript link-grammar printed, which needs no checks.
    pub(crate) const fn from_label(subscript: &str) -> Self {
        let bytes = subscript.as_bytes();
        let mut bits = 0;
        let mut position = 0;
        while position < bytes.len() {
            if bytes[position] != b'*' {
                bits |=
                    (bytes[position] as u64) << (position * LETTER_BITS);
            }
            position += 1;
        }
        Self(bits)
    }

    /// Wraps bits read from a connector descriptor.
    pub(crate) const fn from_bits(bits: u64) -> Self { Self(bits) }

    /// The ASCII letter at `position`, or 0 for a `*` or a position past
    /// the end.
    pub(crate) const fn letter(self, position: usize) -> u8 {
        match position {
            0..POSITIONS => {
                (self.0 >> (position * LETTER_BITS) & LETTER_MASK) as u8
            }
            _ => 0,
        }
    }

    /// `0x7f` at every position that holds a letter.
    const fn letter_mask(self) -> u64 {
        let mut mask = 0;
        let mut position = 0;
        while position < POSITIONS {
            if self.letter(position) != 0 {
                mask |= LETTER_MASK << (position * LETTER_BITS);
            }
            position += 1;
        }
        mask
    }
}

/// `s*s`, with a `*` for each empty position before the last letter.
impl fmt::Display for Subscript {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rest = self.0;
        while rest != 0 {
            match (rest & LETTER_MASK) as u8 {
                0 => f.write_char('*')?,
                letter => f.write_char(letter as char)?,
            }
            rest >>= LETTER_BITS;
        }
        Ok(())
    }
}

/// `Subscript("s*s")`.
impl fmt::Debug for Subscript {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Subscript").field(&self.to_string()).finish()
    }
}
