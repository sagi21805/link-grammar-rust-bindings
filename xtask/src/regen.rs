//! `cargo xtask regen` rewrites the generated files that
//! `link-grammar-sys` commits, so its users need neither libclang nor
//! flex, and then each language's relation enum.

#[path = "../../link-grammar-sys/build/defines.rs"]
mod defines;

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{env, io};

use bindgen::{Bindings, RustEdition, RustTarget};
use extend::ext;
use serde_json::Value;
use thiserror::Error;

use crate::command_ext::{CommandError, CommandExt};
use crate::regen::defines::Target;
use crate::relations::{Relations, RelationsError};

/// The private headers that declare what `link-grammar-sys::internals`
/// holds, relative to `link-grammar/link-grammar`.
const INTERNAL_HEADERS: &[&str] = &[
    "api-structures.h",
    "connectors.h",
    "dict-common/dict-common.h",
    "disjunct-utils.h",
    "linkage/linkage.h",
    "tokenize/tok-structures.h",
];

/// The private structs the safe crate reads.
const INTERNAL_TYPES: &str = "Linkage_s|Link_s|Dictionary_s|\
                              Connector_struct|condesc_struct|\
                              condesc_more_t|hdesc_t|ConTable|\
                              Disjunct_struct|Gword_struct";

/// The structs that [`INTERNAL_TYPES`] only point to and the safe crate
/// never walks. Each becomes a zero-sized opaque struct, so its fields
/// and the types they need stay out of `internals.rs`.
const POINTED_TO_TYPES: &[&str] = &[
    "Afdict_class_struct",
    "Category",
    "Category_cost",
    "Dialect_s",
    "Dict_node_struct",
    "Exp_struct",
    "Parse_Options_s",
    "Postprocessor_s",
    "PP_domains_s",
    "Pool_desc_s",
    "Regex_node_s",
    "Sentence_s",
    "String_id_s",
    "String_set_s",
    "Word_file_struct",
    "Word_struct",
    "anysplit_params",
    "dialect_section_tag",
    "dialect_tag",
    "length_limit_def",
    "pp_knowledge_s",
    "ss_id",
    "ss_slot",
    "str_mem_pool_s",
    "word_queue_s",
];

/// The generated files of `link-grammar-sys`, and the sources they come
/// from.
pub struct Regen {
    workspace: PathBuf,
}

impl Regen {
    /// The generated files of the workspace at `workspace`.
    pub fn new(workspace: PathBuf) -> Self { Self { workspace } }

    /// Regenerates `src/bindings.rs`, `src/internals.rs`,
    /// `generated/pp_lexer.c` and the relation enums.
    pub fn run(&self) -> Result<(), RegenError> {
        let builder = self.builder()?;
        self.bindings(builder.clone())?;
        self.internals(builder)?;
        self.lexer()?;
        // Last, so the enums read the dictionary through the new layouts.
        Relations::new(self.workspace.clone())
            .run()
            .map_err(RegenError::from)
    }

    /// The bindgen settings both runs share, which are the include path,
    /// the bundled build's defines, the Rust target and the formatter.
    fn builder(&self) -> Result<bindgen::Builder, RegenError> {
        let target = Target::host()?;
        let defines = target
            .defines()
            .map(|(name, value)| format!("-D{name}={value}"));
        Ok(bindgen::Builder::default()
            .clang_arg(format!("-I{}", self.sys_include_dir()?.display()))
            .clang_arg(format!("-I{}", self.source_root().display()))
            .clang_args(defines)
            .rust_target(
                RustTarget::stable(85, 0).expect(
                    "1.85 is newer than bindgen's earliest target",
                ),
            )
            .rust_edition(RustEdition::Edition2024)
            .formatter(bindgen::Formatter::Prettyplease))
    }

    /// Runs bindgen over `wrapper.h`, which includes `link-includes.h`.
    /// The output is the same on every target, so `FILE` comes from
    /// `libc` instead of the libc headers' internals, and there are no
    /// layout tests with 64-bit sizes.
    fn bindings(
        &self,
        builder: bindgen::Builder,
    ) -> Result<(), RegenError> {
        builder
            .header(self.sys().join("wrapper.h").display().to_string())
            .allowlist_function(
                "(linkgrammar|lg_error|dictionary|parse_options|sentence|\
                 linkage)_.*",
            )
            .allowlist_type("Cost_Model_type|ConstituentDisplayStyle")
            .opaque_type("(Dictionary|Parse_Options|Sentence|Linkage)_s")
            .blocklist_type("FILE|fpos_t|_IO_.*|__.*")
            .layout_tests(false)
            .generate()?
            .save_to(&self.sys().join("src/bindings.rs"))
    }

    /// Runs bindgen over the private headers, for the layouts of the
    /// structs the safe crate reads.
    fn internals(
        &self,
        builder: bindgen::Builder,
    ) -> Result<(), RegenError> {
        let library = self.source_root().join("link-grammar");
        INTERNAL_HEADERS
            .iter()
            .fold(builder, |builder, header| {
                builder.header(library.join(header).display().to_string())
            })
            .clang_arg(format!("-I{}", library.display()))
            .pointed_to_types_opaque()
            .allowlist_type(INTERNAL_TYPES)
            .allowlist_var("WS_.*|SUBSCRIPT_MARK")
            .blocklist_type("FILE|fpos_t|_IO_.*|__.*|locale_t")
            .raw_line("pub type locale_t = *mut ::std::ffi::c_void;")
            .layout_tests(true)
            .generate()?
            .save_to(&self.sys().join("src/internals.rs"))
    }

    /// Builds `link-grammar-sys` and returns the directory its build
    /// script wrote `link-grammar/link-features.h` to.
    fn sys_include_dir(&self) -> Result<PathBuf, RegenError> {
        let build = Command::cargo()
            .current_dir(&self.workspace)
            .args(["build", "-p", "link-grammar-sys"])
            .arg("--message-format=json")
            .stderr(Stdio::inherit())
            .output_checked()?;
        String::from_utf8_lossy(&build.stdout)
            .lines()
            .filter_map(|line| serde_json::from_str::<Value>(line).ok())
            .find(|message| {
                message["reason"] == "build-script-executed"
                    && message["package_id"]
                        .as_str()
                        .is_some_and(|id| id.contains("link-grammar-sys"))
            })
            .and_then(|message| {
                message["out_dir"].as_str().map(PathBuf::from)
            })
            .map(|out_dir| out_dir.join("include"))
            .ok_or(RegenError::NoBuildScriptOutput)
    }

    /// The `link-grammar-sys` crate.
    fn sys(&self) -> PathBuf { self.workspace.join("link-grammar-sys") }

    /// The submodule, which holds upstream's repository.
    fn source_root(&self) -> PathBuf { self.sys().join("link-grammar") }

    /// Runs flex over the post-processing lexer. The paths are relative
    /// to the workspace and `--noline` drops `#line` directives, so the
    /// output doesn't depend on where the repository is checked out.
    fn lexer(&self) -> Result<(), RegenError> {
        Command::new("flex")
            .current_dir(&self.workspace)
            .args([
                "--noline",
                "-o",
                "link-grammar-sys/generated/pp_lexer.c",
                "link-grammar-sys/link-grammar/link-grammar/post-process/\
                 pp_lexer.l",
            ])
            .run_checked()
            .map_err(RegenError::from)
    }
}

/// Why `cargo xtask regen` stopped.
#[derive(Debug, Error)]
pub enum RegenError {
    #[error(transparent)]
    Command(#[from] CommandError),
    #[error(transparent)]
    Relations(#[from] RelationsError),
    #[error(
        "cargo didn't report where link-grammar-sys's build script wrote"
    )]
    NoBuildScriptOutput,
    #[error(
        "regen runs on Linux and macOS, the hosts the bundled build \
         supports, not on {0}"
    )]
    UnsupportedHost(&'static str),
    #[error("bindgen can't parse link-grammar's headers: {0}")]
    Bindgen(#[from] bindgen::BindgenError),
    #[error("can't write {}: {source}", path.display())]
    Write { path: PathBuf, source: io::Error },
}

impl Target {
    /// The machine regen runs on, which is the target it generates for.
    fn host() -> Result<Self, RegenError> {
        match env::consts::OS {
            "linux" => Ok(Self::LinuxGnu),
            "macos" => Ok(Self::MacOs),
            os => Err(RegenError::UnsupportedHost(os)),
        }
    }
}

#[ext]
impl bindgen::Builder {
    /// Declares each of [`POINTED_TO_TYPES`] as a zero-sized opaque
    /// struct.
    fn pointed_to_types_opaque(self) -> Self {
        POINTED_TO_TYPES.iter().fold(self, |builder, name| {
            builder
                .blocklist_type(format!("{name}(__bindgen_ty_.*)?"))
                .raw_line(format!(
                    "#[repr(C)]\npub struct {name} {{\n    _private: \
                     [u8; 0],\n}}"
                ))
        })
    }
}

#[ext]
impl Bindings {
    /// Writes the bindings to `path`.
    fn save_to(&self, path: &Path) -> Result<(), RegenError> {
        self.write_to_file(path)
            .map_err(|source| RegenError::Write {
                path: path.to_path_buf(),
                source,
            })
    }
}
