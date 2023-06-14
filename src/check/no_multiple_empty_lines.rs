//! Contains the `no-multiple-empty-lines` check.

use crate::CheckResult;

use super::Check;

/// A check that fails if the text contains multiple empty lines in a row.
pub struct NoMultipleEmptyLines {}

impl Check for NoMultipleEmptyLines {
    fn id(&self) -> &'static str {
        "no-multiple-empty-lines"
    }

    fn check(&self, text: &str) -> Option<CheckResult> {
        let mut lines = text.lines().peekable();
        let mut line_no = 0;
        while let Some(line) = lines.next() {
            line_no += 1;
            if line.is_empty() {
                if lines.peek().map_or(false, |l| l.is_empty()) {
                    return Some(CheckResult::from_single_line(
                        self.id(),
                        "multiple empty lines are not allowed".to_string(),
                        line_no,
                        line.to_string(),
                    ));
                }
            }
        }
        None
    }
}
