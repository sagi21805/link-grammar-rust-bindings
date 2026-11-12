//! Repository tasks, run as `cargo xtask <task>`.

#[path = "../../link-grammar-sys/build/link_features.rs"]
mod link_features;
mod regen;

use std::env;
use std::process::ExitCode;

use crate::regen::Regen;

fn main() -> ExitCode {
    let result = match env::args().nth(1).as_deref() {
        Some("regen") => Regen::new().run(),
        _ => {
            eprintln!("usage: cargo xtask regen");
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
