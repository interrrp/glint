//! Tests for the `no-multiple-empty-lines` check.

use glint::{check::NoMultipleEmptyLines, Check, CheckResult};

/// Tests for when there are no empty lines.
#[test]
fn no_empty_lines() {
    let check = NoMultipleEmptyLines {};
    assert_eq!(check.check("Hello, world!"), None);
}

/// Tests for when there is one empty line.
#[test]
fn one_empty_line() {
    let check = NoMultipleEmptyLines {};
    assert_eq!(check.check("Hello,\n\nworld!"), None);
}

/// Tests for when there are multiple empty lines.
#[test]
fn multiple_empty_lines() {
    let check = NoMultipleEmptyLines {};
    assert_eq!(
        check.check("Hello,\n\n\nworld!"),
        Some(CheckResult::from_single_line(
            "no-multiple-empty-lines",
            "multiple empty lines are not allowed".to_string(),
            2,
            "".to_string(),
        ))
    );
}
