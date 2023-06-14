//! Contains the `no-tabs` check.

use super::Check;

/// A check that fails if the text contains any tabs.
pub struct NoTabs {}

impl Check for NoTabs {
    fn id(&self) -> &'static str {
        "no-tabs"
    }

    fn check(&self, text: &str) -> Option<String> {
        if text.contains('\t') {
            Some("tabs are not allowed".to_string())
        } else {
            None
        }
    }
}
