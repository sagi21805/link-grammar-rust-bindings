//! `cargo xtask regen` rewrites the generated files that
//! `link-grammar-sys` commits, so its users need neither libclang nor
//! flex.

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};

use bindgen::{RustEdition, RustTarget};
use serde_json::Value;
use thiserror::Error;

/// The generated files of `link-grammar-sys`, and the sources they come
/// from.
pub struct Regen {
    workspace: PathBuf,
}

impl Regen {
    /// The workspace this xtask was built from.
    pub fn new() -> Self {
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("xtask sits inside the workspace")
            .to_path_buf();
        Self { workspace }
    }

    /// Regenerates `src/bindings.rs` and `generated/pp_lexer.c`.
    pub fn run(&self) -> Result<(), RegenError> {
        self.bindings()?;
        self.lexer()
    }

    /// Runs bindgen over `wrapper.h`, which includes `link-includes.h`.
    /// The output is the same on every target, so `FILE` comes from
    /// `libc` instead of the libc headers' internals, and there are no
    /// layout tests with 64-bit sizes.
    fn bindings(&self) -> Result<(), RegenError> {
        let include_dir = self.sys_include_dir()?;
        let sys = self.workspace.join("link-grammar-sys");
        let source_root = sys.join("link-grammar");
        let output = sys.join("src/bindings.rs");
        bindgen::Builder::default()
            .header(sys.join("wrapper.h").display().to_string())
            .clang_arg(format!("-I{}", include_dir.display()))
            .clang_arg(format!("-I{}", source_root.display()))
            .rust_target(
                RustTarget::stable(85, 0)
                    .expect("1.85 is newer than bindgen's earliest target"),
            )
            .rust_edition(RustEdition::Edition2024)
            .formatter(bindgen::Formatter::Prettyplease)
            .allowlist_function(
                "(linkgrammar|lg_error|dictionary|parse_options|sentence|\
                 linkage)_.*",
            )
            .allowlist_type("Cost_Model_type|ConstituentDisplayStyle")
            .opaque_type("(Dictionary|Parse_Options|Sentence|Linkage)_s")
            .blocklist_type("FILE|_IO_.*|__.*")
            .layout_tests(false)
            .generate()?
            .write_to_file(&output)
            .map_err(|source| RegenError::Write { path: output, source })
    }

    /// Builds `link-grammar-sys` and returns the directory its build
    /// script wrote `link-grammar/link-features.h` to.
    fn sys_include_dir(&self) -> Result<PathBuf, RegenError> {
        let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
        let build = Command::new(cargo)
            .current_dir(&self.workspace)
            .args(["build", "-p", "link-grammar-sys"])
            .arg("--message-format=json")
            .stderr(Stdio::inherit())
            .output()
            .map_err(RegenError::CargoMissing)?;
        if !build.status.success() {
            return Err(RegenError::SysBuild(build.status));
        }
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

    /// Runs flex over the post-processing lexer. The paths are relative
    /// to the workspace and `--noline` drops `#line` directives, so the
    /// output doesn't depend on where the repository is checked out.
    fn lexer(&self) -> Result<(), RegenError> {
        let generated = self.workspace.join("link-grammar-sys/generated");
        fs::create_dir_all(&generated).map_err(|source| {
            RegenError::Write {
                path: generated,
                source,
            }
        })?;
        let status = Command::new("flex")
            .current_dir(&self.workspace)
            .args([
                "--noline",
                "-o",
                "link-grammar-sys/generated/pp_lexer.c",
                "link-grammar-sys/link-grammar/link-grammar/post-process/\
                 pp_lexer.l",
            ])
            .status()
            .map_err(RegenError::FlexMissing)?;
        status
            .success()
            .then_some(())
            .ok_or(RegenError::Flex(status))
    }
}

/// Why `cargo xtask regen` stopped.
#[derive(Debug, Error)]
pub enum RegenError {
    #[error("can't run cargo: {0}")]
    CargoMissing(#[source] io::Error),
    #[error("building link-grammar-sys failed with {0}")]
    SysBuild(ExitStatus),
    #[error(
        "cargo didn't report where link-grammar-sys's build script wrote"
    )]
    NoBuildScriptOutput,
    #[error("bindgen can't parse link-includes.h: {0}")]
    Bindgen(#[from] bindgen::BindgenError),
    #[error("can't write {}: {source}", path.display())]
    Write { path: PathBuf, source: io::Error },
    #[error("can't run flex, which must be on PATH: {0}")]
    FlexMissing(#[source] io::Error),
    #[error("flex failed with {0}")]
    Flex(ExitStatus),
}
