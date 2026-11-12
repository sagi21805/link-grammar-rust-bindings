# link-grammar for Rust

Rust bindings for [link-grammar](https://github.com/opencog/link-grammar),
the CMU/OpenCog syntactic parser for English and other languages.

| Crate | What it is |
|---|---|
| `link-grammar-sys` | Raw FFI bindings to the C API in `link-includes.h` |
| `link-grammar` | A safe API over `link-grammar-sys` |

## Building

By default, `link-grammar-sys` builds the C library itself from the
upstream sources it ships, so `cargo add link-grammar` needs only a C
compiler. You don't run `./configure && make`, and you don't need libclang
or flex. The English dictionary ships with the crate and loads without any
setup. Point `Dictionary::set_data_dir` at a dictionary tree to load other
languages.

To link against a link-grammar you installed yourself, turn off the
`bundled` feature. The build then finds the library through pkg-config.

```toml
[dependencies]
link-grammar = { version = "0.1", default-features = false }
```

The bundled build supports Linux with glibc for now. On other targets,
install link-grammar and turn off `bundled`.

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

To check that the crate builds from the files it publishes, the way
crates.io users get it, run:

```sh
cargo xtask package-check
```

It runs `cargo package`, unpacks the `.crate` under
`target/package-check`, and parses a sentence with it.

## License

LGPL-2.1, the same as link-grammar.

The `bundled` build links link-grammar statically into your program. The
LGPL then requires that the people you distribute the program to can
relink it against a modified link-grammar. You can do that by shipping
your object files or your source code, or by turning off `bundled` and
linking the system's shared library instead. Read the LGPL's section 6
before you distribute a statically linked binary.
