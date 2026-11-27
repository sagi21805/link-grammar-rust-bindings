# link-grammar-sys

Raw FFI bindings to the C API of
[link-grammar](https://github.com/opencog/link-grammar), from
`link-includes.h`. Most programs want the safe
[`link-grammar`](https://crates.io/crates/link-grammar) crate instead.

## Building

The build compiles link-grammar 5.13.2 from the sources this crate ships
and links it statically. The English dictionary ships too, and the library
finds it with no setup. You need only a C compiler, because the bindings
and the post-processing lexer come pre-generated. The build supports Linux
with glibc and macOS for now.

## License

LGPL-2.1, the same as link-grammar. The build links link-grammar statically,
so read the LGPL's section 6 before you distribute a binary.
