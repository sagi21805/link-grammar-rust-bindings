# link-grammar-sys

Raw FFI bindings to the C API of
[link-grammar](https://github.com/opencog/link-grammar), from
`link-includes.h`. Most programs want the safe
[`link-grammar`](https://crates.io/crates/link-grammar) crate instead.

## Building

The default `bundled` feature compiles link-grammar 5.13.2 from the
sources this crate ships and links it statically. The English dictionary
ships too, and the library finds it with no setup. You need only a C
compiler, because the bindings and the post-processing lexer come
pre-generated. The bundled build supports Linux with glibc for now.

Without `bundled`, the build finds a system install through pkg-config
and links that.

With `bundled`, crates that depend on this one get two variables from
its build script.
`DEP_LINK_GRAMMAR_INCLUDE` is the directory holding the generated
`link-grammar/link-features.h`, and `DEP_LINK_GRAMMAR_DATA` is the
bundled dictionary directory.

## License

LGPL-2.1, the same as link-grammar. The `bundled` build links link-grammar
statically, so read the LGPL's section 6 before you distribute a binary.
