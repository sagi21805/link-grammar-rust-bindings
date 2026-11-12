use std::env;
use std::path::PathBuf;

fn main() {
    let source = LinkGrammarSource::locate();
    source.link();
    source.generate_bindings();
}

/// A link-grammar source tree that `configure && make` already built.
struct LinkGrammarSource {
    root: PathBuf,
}

impl LinkGrammarSource {
    /// The tree named by `LINK_GRAMMAR_DIR`, or the crate's
    /// `link-grammar` submodule.
    fn locate() -> Self {
        println!("cargo:rerun-if-env-changed=LINK_GRAMMAR_DIR");
        let root = env::var_os("LINK_GRAMMAR_DIR").map_or_else(
            || {
                let manifest = env::var_os("CARGO_MANIFEST_DIR")
                    .expect("cargo sets CARGO_MANIFEST_DIR");
                PathBuf::from(manifest).join("link-grammar")
            },
            PathBuf::from,
        );
        Self { root }
    }

    /// The directory libtool writes the built libraries to.
    fn lib_dir(&self) -> PathBuf { self.root.join("link-grammar/.libs") }

    /// Links the static library and the libraries it depends on.
    fn link(&self) {
        let lib_dir = self.lib_dir();
        let archive = lib_dir.join("liblink-grammar.a");
        assert!(
            archive.exists(),
            "{} is missing. Run `./configure && make` in {} first.",
            archive.display(),
            self.root.display(),
        );
        println!("cargo:rerun-if-changed={}", archive.display());
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-link-lib=static=link-grammar");

        pkg_config::probe_library("libpcre2-8")
            .expect("link-grammar needs pcre2, found through pkg-config");
        println!("cargo:rustc-link-lib=dylib=sqlite3");
        let cpp = match env::var("CARGO_CFG_TARGET_VENDOR").as_deref() {
            Ok("apple") => "c++",
            _ => "stdc++",
        };
        println!("cargo:rustc-link-lib=dylib={cpp}");
    }

    /// Runs bindgen over the public API in `link-includes.h`.
    fn generate_bindings(&self) {
        println!("cargo:rerun-if-changed=wrapper.h");
        let out_dir = PathBuf::from(
            env::var_os("OUT_DIR").expect("cargo sets OUT_DIR"),
        );
        bindgen::Builder::default()
            .header("wrapper.h")
            .clang_arg(format!("-I{}", self.root.display()))
            .rust_edition(bindgen::RustEdition::Edition2024)
            .allowlist_function(
                "(linkgrammar|lg_error|dictionary|parse_options|sentence|\
                 linkage)_.*",
            )
            .allowlist_type("Cost_Model_type|ConstituentDisplayStyle")
            .opaque_type("(Dictionary|Parse_Options|Sentence|Linkage)_s")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
            .expect("bindgen can parse link-includes.h")
            .write_to_file(out_dir.join("bindings.rs"))
            .expect("OUT_DIR is writable");
    }
}
