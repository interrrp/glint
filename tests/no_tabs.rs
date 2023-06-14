//! Tests for the `no-tabs` check.

use glint::{check::NoTabs, Check, CheckResult};

/// Tests for when there are no tabs.
#[test]
fn no_tabs() {
    let check = NoTabs {};
    assert_eq!(check.check("Hello, world!"), None);
}

/// Tests for when there is one tab.
#[test]
fn one_tab() {
    let check = NoTabs {};
    assert_eq!(
        check.check("Hello,\tworld!"),
        Some(CheckResult::from_single_line(
            "no-tabs",
            "tabs are not allowed".to_string(),
            1,
            "Hello,\tworld!".to_string(),
        ))
    );
}

/// Tests for when there are multiple tabs.
#[test]
fn multiple_tabs() {
    let check = NoTabs {};
    assert_eq!(
        check.check("Hello,\t \tworld!"),
        Some(CheckResult::from_single_line(
            "no-tabs",
            "tabs are not allowed".to_string(),
            1,
            "Hello,\t \tworld!".to_string(),
        ))
    );
}
