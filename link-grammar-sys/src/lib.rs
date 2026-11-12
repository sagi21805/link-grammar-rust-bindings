//! Raw FFI bindings to link-grammar's public C API in `link-includes.h`.
//! The `link-grammar` crate wraps them in a safe API.

#![expect(
    non_camel_case_types,
    non_upper_case_globals,
    reason = "bindgen keeps the C names"
)]

pub use libc::FILE;

// `cargo xtask regen` writes this file with bindgen.
include!("bindings.rs");
