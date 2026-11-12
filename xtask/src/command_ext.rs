//! Generic extensions for running commands.

use std::env;
use std::io;
use std::process::{Command, ExitStatus, Output};

use extend::ext;
use thiserror::Error;

#[ext(name = CommandExt)]
pub impl Command {
    /// The cargo that runs this xtask, or the one on `PATH`.
    fn cargo() -> Command {
        Command::new(
            env::var_os("CARGO").unwrap_or_else(|| "cargo".into()),
        )
    }

    /// Runs the command and fails unless it exits successfully.
    fn run_checked(&mut self) -> Result<(), CommandError> {
        let status =
            self.status().map_err(|source| CommandError::Spawn {
                program: self.program_name(),
                source,
            })?;
        self.check(status)
    }

    /// Runs the command, captures its stdout, and fails unless it exits
    /// successfully.
    fn output_checked(&mut self) -> Result<Output, CommandError> {
        let output =
            self.output().map_err(|source| CommandError::Spawn {
                program: self.program_name(),
                source,
            })?;
        self.check(output.status)?;
        Ok(output)
    }

    /// Fails unless `status` is a successful exit.
    fn check(&self, status: ExitStatus) -> Result<(), CommandError> {
        status.success().then_some(()).ok_or_else(|| {
            CommandError::Failed {
                program: self.program_name(),
                status,
            }
        })
    }

    /// The program, for error messages.
    fn program_name(&self) -> String {
        self.get_program().to_string_lossy().into_owned()
    }
}

/// Why a command didn't run to a successful exit.
#[derive(Debug, Error)]
pub enum CommandError {
    #[error("can't run {program}, which must be on PATH: {source}")]
    Spawn { program: String, source: io::Error },
    #[error("{program} failed with {status}")]
    Failed { program: String, status: ExitStatus },
}
