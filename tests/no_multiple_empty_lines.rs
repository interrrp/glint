//! Tests for the `no-multiple-empty-lines` check.

use glint::{check::NoMultipleEmptyLines, Check};

/// Test for when there are no empty lines.
#[test]
fn no_empty_lines() {
    let check = NoMultipleEmptyLines {};
    assert_eq!(check.check("Hello, world!"), None);
}

/// Test for when there is one empty line.
#[test]
fn one_empty_line() {
    let check = NoMultipleEmptyLines {};
    assert_eq!(check.check("Hello,\nworld!"), None);
}

/// Test for when there are multiple empty lines.
#[test]
fn multiple_empty_lines() {
    let check = NoMultipleEmptyLines {};
    assert_eq!(
        check.check("Hello,\n\n\nworld!"),
        Some("multiple empty lines are not allowed".to_string())
    );
}
