# link-grammar for Rust

> [!WARNING]
> This repository is under heavy development, and each commit currently may include a breaking change.

Rust bindings for [link-grammar](https://github.com/opencog/link-grammar),
the CMU/OpenCog syntactic parser for English and other languages.

| Crate | What it is |
|---|---|
| `link-grammar-sys` | Raw FFI bindings to the C API in `link-includes.h` |
| `link-grammar` | A safe API over `link-grammar-sys` |

## Building

[`link-grammar/README.md`](link-grammar/README.md) covers the build and
the dictionaries.

## Contributing

The upstream sources are a git submodule in
`link-grammar-sys/link-grammar`, pinned to the
`link-grammar-5.13.2` tag. Fetch it after cloning:

```sh
git submodule update --init
```

`link-grammar-sys` commits `src/bindings.rs` from bindgen and
`generated/pp_lexer.c` from flex, so users need neither libclang nor flex.
After moving the submodule to a new tag, regenerate both with libclang and
flex installed.

```sh
cargo xtask regen
```

Each language's relation enum is generated from its dictionary, with one
variant per link type. `regen` writes them too. After the generator or a
dictionary changes, write them on their own, which needs neither libclang
nor flex.

```sh
cargo xtask relations
```

To check that the crate builds from the files it publishes, the way
crates.io users get it, run:

```sh
cargo xtask package-check
```

It runs `cargo package`, unpacks the `.crate` under
`target/package-check`, and parses a sentence with it.

## License

LGPL-2.1, the same as link-grammar.

The build links link-grammar statically into your program. The
LGPL then requires that the people you distribute the program to can
relink it against a modified link-grammar. You can do that by shipping
your object files or your source code. Read the LGPL's section 6 before
you distribute a statically linked binary.
