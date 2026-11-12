//! Raw FFI bindings to link-grammar's public C API in `link-includes.h`.
//! The `link-grammar` crate wraps them in a safe API.
//!
//! The default `bundled` feature compiles link-grammar from the sources
//! this crate ships and links it statically. Without it, the build finds
//! a system install through pkg-config and links that.

#![expect(
    non_camel_case_types,
    non_upper_case_globals,
    reason = "bindgen keeps the C names"
)]

pub use libc::FILE;

/// The dictionaries that ship with the crate, one directory per
/// language. Only English (`en`) is included. The bundled library
/// searches here when no data directory is set, so
/// `dictionary_create_lang(c"en".as_ptr())` works with no setup.
#[cfg(feature = "bundled")]
pub const BUNDLED_DATA_DIR: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/link-grammar/data");

// `cargo xtask regen` writes this file with bindgen.
include!("bindings.rs");
