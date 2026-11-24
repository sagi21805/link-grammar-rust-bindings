//! `cargo xtask package-check` builds `link-grammar-sys` from the files
//! `cargo package` puts in its `.crate`, the way a crates.io user gets it,
//! and parses a sentence with it.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use thiserror::Error;

use crate::command_ext::{CommandError, CommandExt};

/// The scratch program that uses the packaged crate.
const SCRATCH_MAIN: &str = include_str!("../package-check/main.rs");

/// A check of `link-grammar-sys` as published, run in a scratch
/// directory under `target/`.
pub struct PackageCheck {
    workspace: PathBuf,
    dir: PathBuf,
}

impl PackageCheck {
    /// The check for the workspace this xtask was built from.
    pub fn new() -> Self {
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("xtask sits inside the workspace")
            .to_path_buf();
        let dir = workspace.join("target/package-check");
        Self { workspace, dir }
    }

    /// Packages, unpacks, builds and runs, starting from an empty
    /// scratch directory.
    pub fn run(&self) -> Result<(), PackageCheckError> {
        if self.dir.exists() {
            fs::remove_dir_all(&self.dir).map_err(self.io_error())?;
        }
        let packaged = self.package()?;
        let unpacked = self.unpack(&packaged)?;
        self.write_scratch_crate(&unpacked)?;
        Command::cargo()
            .current_dir(self.scratch())
            .args(["run", "--quiet"])
            .run_checked()?;
        Ok(())
    }

    /// Runs `cargo package` and returns the `.crate` it wrote.
    fn package(&self) -> Result<PathBuf, PackageCheckError> {
        Command::cargo()
            .current_dir(&self.workspace)
            .args(["package", "-p", "link-grammar-sys"])
            .args(["--allow-dirty", "--no-verify", "--target-dir"])
            .arg(&self.dir)
            .run_checked()?;
        let package_dir = self.dir.join("package");
        fs::read_dir(&package_dir)
            .map_err(self.io_error())?
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .find(|path| {
                path.extension().is_some_and(|ext| ext == "crate")
            })
            .ok_or(PackageCheckError::NoCrateFile(package_dir))
    }

    /// Extracts `packaged` and returns the crate directory inside it.
    fn unpack(
        &self,
        packaged: &Path,
    ) -> Result<PathBuf, PackageCheckError> {
        let unpacked = self.dir.join("unpacked");
        fs::create_dir_all(&unpacked).map_err(self.io_error())?;
        Command::new("tar")
            .arg("-xzf")
            .arg(packaged)
            .arg("-C")
            .arg(&unpacked)
            .run_checked()?;
        let name =
            packaged.file_stem().expect("the .crate file has a name");
        Ok(unpacked.join(name))
    }

    /// Writes a crate that depends on the unpacked `link-grammar-sys` by
    /// path. Its empty `[workspace]` keeps it out of this workspace.
    fn write_scratch_crate(
        &self,
        unpacked: &Path,
    ) -> Result<(), PackageCheckError> {
        let src = self.scratch().join("src");
        fs::create_dir_all(&src).map_err(self.io_error())?;
        let manifest = format!(
            "[package]\nname = \"package-check\"\nversion = \
             \"0.0.0\"\nedition = \
             \"2024\"\n\n[dependencies]\nlink-grammar-sys = {{ path = \
             '{}' }}\n\n[workspace]\n",
            unpacked.display()
        );
        fs::write(self.scratch().join("Cargo.toml"), manifest)
            .and_then(|()| fs::write(src.join("main.rs"), SCRATCH_MAIN))
            .map_err(self.io_error())
    }

    /// The scratch crate's directory.
    fn scratch(&self) -> PathBuf { self.dir.join("scratch") }

    /// Wraps an I/O error in the scratch directory.
    fn io_error(&self) -> impl Fn(io::Error) -> PackageCheckError + '_ {
        |source| PackageCheckError::Io {
            dir: self.dir.clone(),
            source,
        }
    }
}

/// Why `cargo xtask package-check` stopped.
#[derive(Debug, Error)]
pub enum PackageCheckError {
    #[error(transparent)]
    Command(#[from] CommandError),
    #[error("can't use the scratch directory {}: {source}", dir.display())]
    Io { dir: PathBuf, source: io::Error },
    #[error("cargo package wrote no .crate file to {}", .0.display())]
    NoCrateFile(PathBuf),
}
