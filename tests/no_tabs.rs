//! Tests for the `no-tabs` check.

use glint::{check::NoTabs, Check};

/// Test for when there are no tabs.
#[test]
fn no_tabs() {
    let check = NoTabs {};
    assert_eq!(check.check("Hello, world!"), None);
}

/// Test for when there is one tab.
#[test]
fn one_tab() {
    let check = NoTabs {};
    assert_eq!(
        check.check("Hello,\tworld!"),
        Some("tabs are not allowed".to_string())
    );
}

/// Test for when there are multiple tabs.
#[test]
fn multiple_tabs() {
    let check = NoTabs {};
    assert_eq!(
        check.check("Hello,\t \tworld!"),
        Some("tabs are not allowed".to_string())
    );
}
