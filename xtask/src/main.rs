//! Repository tasks, run as `cargo xtask <task>`.

mod command_ext;
mod package_check;
mod regen;

use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::package_check::PackageCheck;
use crate::regen::Regen;

fn main() -> ExitCode {
    let result: Result<(), Box<dyn Error>> = match env::args()
        .nth(1)
        .as_deref()
    {
        Some("regen") => Regen::new(workspace()).run().map_err(Into::into),
        Some("package-check") => {
            PackageCheck::new(workspace()).run().map_err(Into::into)
        }
        _ => {
            eprintln!("usage: cargo xtask <regen | package-check>");
            return ExitCode::FAILURE;
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

/// The workspace this xtask was built from.
fn workspace() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask sits inside the workspace")
        .to_path_buf()
}
