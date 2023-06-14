//! Contains the `CheckResult` struct.

use std::fmt::Display;

/// A result of a failing check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckResult {
    /// The ID of the check.
    pub id: &'static str,

    /// The error message of the check.
    pub error: String,

    /// The starting line of the error.
    pub start_line: usize,
    /// The starting column of the error.
    pub start_column: usize,
    /// The ending line of the error.
    pub end_line: usize,
    /// The ending column of the error.
    pub end_column: usize,
}

impl CheckResult {
    /// Creates a new `CheckResult` from a single line.
    pub fn from_single_line(id: &'static str, error: String, line_no: usize, line: String) -> Self {
        Self {
            id,
            error,
            start_line: line_no,
            start_column: 0,
            end_line: line_no,
            end_column: line.len(),
        }
    }
}

impl Display for CheckResult {
    /// Formats the check result.
    ///
    /// This will return a string in the following format:
    ///
    /// ```text
    /// {id}:{start_line}:{start_column}:{end_line}:{end_column}: {error}
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}:{}: {}",
            self.id, self.start_line, self.start_column, self.end_line, self.end_column, self.error
        )
    }
}
