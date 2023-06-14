//! Contains general/base functionality for checks.

pub use no_multiple_empty_lines::NoMultipleEmptyLines;
pub use no_tabs::NoTabs;

mod no_multiple_empty_lines;
mod no_tabs;

/// A list of all checks. This is used by `check_all`.
pub const CHECKS: &[&dyn Check] = &[&NoTabs {}, &NoMultipleEmptyLines {}];

/// A check.
///
/// A check checks given text for a certain condition. If the condition is met, the check passes (
/// returns `None` from `check`). If the condition is not met, the check fails (returns a `String`
/// containing the error message from `check`).
pub trait Check {
    /// Gets the ID of the check.
    ///
    /// Stylistically, the ID should be in kebab-case (all lowercase, words separated by hyphens).
    fn id(&self) -> &'static str;

    /// Checks the given text.
    ///
    /// If the check fails, return a `String` with the appropriate error message.
    fn check(&self, text: &str) -> Option<String>;
}

/// Checks the given text with all checks.
///
/// This returns all errors from all checks. If there are no errors, this returns an empty `Vec`.
/// Every error is a `String` containing the error message from the check, prefixed with the check
/// ID and a colon (e.g. `no-tabs: tabs are not allowed`).
pub fn check_all(text: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for check in CHECKS {
        if let Some(err) = check.check(text) {
            errors.push(format!("{}: {}", check.id(), err));
        }
    }
    errors
}
