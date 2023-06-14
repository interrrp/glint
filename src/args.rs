//! Contains the CLI arguments (`Args`).

use std::path::PathBuf;

use clap::Parser;

/// A general-purpose text linter.
#[derive(Parser)]
pub struct Args {
    /// The path(s) to the file(s) to lint.
    pub paths: Vec<PathBuf>,
}
