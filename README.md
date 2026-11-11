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

## Contributing

The upstream sources are a git submodule in
`crates/link-grammar-sys/link-grammar`, pinned to the
`link-grammar-5.13.2` tag. Fetch it after cloning:

```sh
git submodule update --init
```

## License

LGPL-2.1, the same as link-grammar.
