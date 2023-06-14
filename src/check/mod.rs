//! Contains general/base functionality for checks.

pub use no_multiple_empty_lines::NoMultipleEmptyLines;
pub use no_tabs::NoTabs;
pub use result::CheckResult;

mod no_multiple_empty_lines;
mod no_tabs;
mod result;

/// A list of all checks. This is used by `check_all`.
pub const CHECKS: &[&dyn Check] = &[&NoTabs {}, &NoMultipleEmptyLines {}];

/// A check.
///
/// A check checks given text for a certain condition. If the condition is met, the check passes (
/// returns `None` from `check`). If the condition is not met, the check fails (returns a
/// `CheckResult` containing the error message from `check`).
pub trait Check {
    /// Gets the ID of the check.
    ///
    /// Stylistically, the ID should be in kebab-case (all lowercase, words separated by hyphens).
    fn id(&self) -> &'static str;

    /// Checks the given text.
    ///
    /// If the check fails, return a `CheckResult` with the appropriate info.
    fn check(&self, text: &str) -> Option<CheckResult>;
}

/// Checks the given text with all checks.
///
/// This will return a list of all errors found in the text.
pub fn check_all(text: &str) -> Vec<CheckResult> {
    CHECKS
        .iter()
        .filter_map(|check| check.check(text))
        .collect()
}
