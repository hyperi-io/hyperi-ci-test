//! Project:      ci-test-rust-simple
//! File:         tests/integration_test.rs
//! Purpose:      Integration tests for the CLI binary
//! Language:     Rust
//!
//! License:      BUSL-1.1
//! Copyright:    (c) 2026 HYPERI PTY LIMITED

use std::process::Command;

fn cargo_bin() -> Command {
    let mut cmd = Command::new(env!("CARGO"));
    cmd.args(["run", "--quiet", "--"]);
    cmd
}

#[test]
fn test_default_output() {
    let output = cargo_bin().output().expect("failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hello from ci-test-rust-simple!"));
}

#[test]
fn test_version_flag() {
    let output = cargo_bin()
        .arg("--version")
        .output()
        .expect("failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected = format!("ci-test-rust-simple {}", env!("CARGO_PKG_VERSION"));
    assert!(
        stdout.contains(&expected),
        "expected version string '{}' in output: {}",
        expected,
        stdout
    );
}

#[test]
fn test_help_flag() {
    let output = cargo_bin()
        .arg("--help")
        .output()
        .expect("failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("--help"));
    assert!(stdout.contains("--version"));
}

#[test]
fn test_unknown_option_fails() {
    let output = cargo_bin()
        .arg("--invalid")
        .output()
        .expect("failed to execute binary");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("error: unknown option '--invalid'"));
}
