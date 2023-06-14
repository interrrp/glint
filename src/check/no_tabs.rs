//! Contains the `no-tabs` check.

use crate::CheckResult;

use super::Check;

/// A check that fails if the text contains any tabs.
pub struct NoTabs {}

impl Check for NoTabs {
    fn id(&self) -> &'static str {
        "no-tabs"
    }

    fn check(&self, text: &str) -> Option<CheckResult> {
        if text.contains('\t') {
            Some(CheckResult::from_single_line(
                self.id(),
                "tabs are not allowed".to_string(),
                1,
                text.lines().next().unwrap().to_string(),
            ))
        } else {
            None
        }
    }
}
