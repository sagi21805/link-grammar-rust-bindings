//! Compiles link-grammar from the submodule with the `bundled` feature,
//! or finds a system install through pkg-config without it.

mod link_features;

use std::env;
use std::path::{Path, PathBuf};

use crate::link_features::LinkFeatures;

fn main() {
    match env::var_os("CARGO_FEATURE_BUNDLED") {
        Some(_) => LinkGrammarBuild::new().run(),
        None => {
            if let Err(error) = pkg_config::probe_library("link-grammar") {
                panic!(
                    "{error}\nInstall link-grammar, or turn on the \
                     `bundled` feature to compile it from source."
                );
            }
        }
    }
}

/// The `.c` entries of `liblink_grammar_la_SOURCES` in
/// `link-grammar/Makefile.am`, in its order. The C++ files of
/// `dict-atomese/` are left out, and `generated/pp_lexer.c` replaces
/// `post-process/pp_lexer.l`. The SQLite, Atomese, spell-checker and
/// wordgraph-display files compile to nothing without their defines.
const SOURCES: &[&str] = &[
    "config.c",
    "connectors.c",
    "dict-common/dialect.c",
    "dict-common/dict-affix-impl.c",
    "dict-common/dict-common.c",
    "dict-common/dict-internals.c",
    "dict-common/dict-locale.c",
    "dict-common/dict-utils.c",
    "dict-common/file-utils.c",
    "dict-common/idiom.c",
    "dict-common/print-dict.c",
    "dict-common/regex-morph.c",
    "dict-file/read-dialect.c",
    "dict-file/dictionary.c",
    "dict-file/read-dict.c",
    "dict-file/read-regex.c",
    "dict-file/word-file.c",
    "dict-ram/dict-ram.c",
    "dict-sql/read-sql.c",
    "dict-atomese/read-atomese.c",
    "disjunct-utils.c",
    "error.c",
    "linkage/analyze-linkage.c",
    "linkage/freeli.c",
    "linkage/linkage.c",
    "linkage/lisjuncts.c",
    "linkage/sane.c",
    "linkage/score.c",
    "memory-pool.c",
    "options.c",
    "parse/count.c",
    "parse/extract-links.c",
    "parse/fast-match.c",
    "parse/histogram.c",
    "parse/parse.c",
    "parse/preparation.c",
    "parse/prune.c",
    "post-process/constituents.c",
    "post-process/post-process.c",
    "post-process/pp_knowledge.c",
    "post-process/pp_linkset.c",
    "prepare/build-disjuncts.c",
    "prepare/exprune.c",
    "print/print.c",
    "print/print-util.c",
    "print/wcwidth.c",
    "resources.c",
    "sentence.c",
    "string-set.c",
    "string-id.c",
    "tokenize/anysplit.c",
    "tokenize/lookup-exprs.c",
    "tokenize/spellcheck-aspell.c",
    "tokenize/spellcheck-hun.c",
    "tokenize/tokenize.c",
    "tokenize/wg-display.c",
    "tokenize/wordgraph.c",
    "tracon-set.c",
    "utilities.c",
];

/// The bundled build, which compiles link-grammar's C sources with `cc`
/// into a static library.
struct LinkGrammarBuild {
    crate_dir: PathBuf,
    out_dir: PathBuf,
    target: Target,
}

impl LinkGrammarBuild {
    fn new() -> Self {
        Self {
            crate_dir: env_path("CARGO_MANIFEST_DIR"),
            out_dir: env_path("OUT_DIR"),
            target: Target::from_env(),
        }
    }

    /// Writes `link-features.h`, compiles and links the library, and
    /// tells crates that depend on this one where its headers and
    /// dictionaries are.
    fn run(&self) {
        LinkFeatures::new(
            &self.source_root(),
            self.target.host_os(),
            &self.target.lg_defs(),
        )
        .and_then(|features| features.write_to(&self.include_dir()))
        .expect(
            "the submodule has link-features.h.in and configure.ac, and \
             OUT_DIR is writable",
        );
        self.compile();

        println!(
            "cargo::metadata=include={}",
            self.include_dir().display()
        );
        println!("cargo::metadata=data={}", self.data_dir().display());
        println!("cargo::rerun-if-changed=build");
        println!("cargo::rerun-if-changed=generated");
        println!("cargo::rerun-if-changed=link-grammar/link-grammar");
    }

    /// Compiles `SOURCES` and the generated lexer into
    /// `liblink-grammar.a`, and links it.
    fn compile(&self) {
        let lib_dir = self.source_root().join("link-grammar");
        let mut build = cc::Build::new();
        build
            .files(SOURCES.iter().map(|source| lib_dir.join(source)))
            .file(self.crate_dir.join("generated/pp_lexer.c"))
            .include(self.include_dir())
            .include(self.source_root())
            .include(&lib_dir)
            .include(lib_dir.join("post-process"))
            .std("c11")
            .define(
                "DICTIONARY_DIR",
                c_string_literal(&self.data_dir()).as_str(),
            )
            .warnings(false);
        for (name, value) in self.target.defines() {
            build.define(name, *value);
        }
        build.compile("link-grammar");
    }

    /// The submodule, which holds upstream's repository.
    fn source_root(&self) -> PathBuf {
        self.crate_dir.join("link-grammar")
    }

    /// Where `link-grammar/link-features.h` is written. `cargo xtask
    /// regen` gives this directory to bindgen too.
    fn include_dir(&self) -> PathBuf { self.out_dir.join("include") }

    /// The dictionaries, one directory per language. The library looks
    /// here when a program doesn't set its own data directory.
    fn data_dir(&self) -> PathBuf { self.source_root().join("data") }
}

/// A target whose configure results the bundled build knows.
enum Target {
    LinuxGnu,
}

impl Target {
    fn from_env() -> Self {
        let os = env::var("CARGO_CFG_TARGET_OS")
            .expect("cargo sets CARGO_CFG_TARGET_OS");
        let libc = env::var("CARGO_CFG_TARGET_ENV")
            .expect("cargo sets CARGO_CFG_TARGET_ENV");
        match (os.as_str(), libc.as_str()) {
            ("linux", "gnu") => Self::LinuxGnu,
            _ => panic!(
                "The bundled link-grammar build doesn't support the \
                 {os}-{libc} target yet. Install link-grammar and turn \
                 off the `bundled` feature with `default-features = \
                 false`."
            ),
        }
    }

    /// The OS name configure reports in
    /// `linkgrammar_get_configuration()`.
    fn host_os(&self) -> &'static str {
        match self {
            Self::LinuxGnu => "linux-gnu",
        }
    }

    /// The defines configure finds on this target, with the optional
    /// libraries turned off.
    fn defines(&self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::LinuxGnu => LINUX_GNU_DEFINES,
        }
    }

    /// `defines` as configure's `LG_DEFS`, the `-DNAME=VALUE` list that
    /// `linkgrammar_get_configuration()` reports.
    fn lg_defs(&self) -> String {
        self.defines()
            .iter()
            .map(|(name, value)| format!("-D{name}={value}"))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// What configure finds on glibc Linux, minus the macros no compiled
/// source reads. Regex comes from libc's `<regex.h>`.
const LINUX_GNU_DEFINES: &[(&str, &str)] = &[
    ("_DEFAULT_SOURCE", "1"),
    ("_GNU_SOURCE", "1"),
    ("TLS", "_Thread_local"),
    ("HAVE_REGEX_H", "1"),
    ("HAVE_LOCALE_T_IN_LOCALE_H", "1"),
    ("HAVE_ALLOCA_H", "1"),
    ("HAVE_ALIGNED_ALLOC", "1"),
    ("HAVE___BUILTIN_TYPES_COMPATIBLE_P", "1"),
    ("HAVE_TYPEOF", "1"),
    ("typeof", "__typeof__"),
    ("HAVE_STRNDUP", "1"),
    ("HAVE_STRTOK_R", "1"),
    ("HAVE_STRERROR_R", "1"),
    ("STRERROR_R_CHAR_P", "1"),
    ("HAVE_MALLOC_TRIM", "1"),
];

/// The path in the environment variable `name`, which cargo sets.
fn env_path(name: &str) -> PathBuf {
    env::var_os(name)
        .map(PathBuf::from)
        .expect("cargo sets CARGO_MANIFEST_DIR and OUT_DIR")
}

/// `path` as a C string literal.
fn c_string_literal(path: &Path) -> String {
    let path = path.to_str().expect("the crate's path is valid UTF-8");
    format!("\"{}\"", path.replace('\\', "\\\\").replace('"', "\\\""))
}
