//! Contains the `no-multiple-empty-lines` check.

use super::Check;

/// A check that fails if the text contains multiple empty lines in a row.
pub struct NoMultipleEmptyLines {}

impl Check for NoMultipleEmptyLines {
    fn id(&self) -> &'static str {
        "no-multiple-empty-lines"
    }

    fn check(&self, text: &str) -> Option<String> {
        if text.contains("\n\n\n") {
            Some("multiple empty lines are not allowed".to_string())
        } else {
            None
        }
    }
}
