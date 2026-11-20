// Project:   ci-test-rust-workspace-cli
// License:   FSL-1.1-ALv2
// Copyright: (c) 2026 HYPERI PTY LIMITED

//! Workspace cli binary. The bin's only purpose is to prove that:
//!
//! 1. The sibling member crate (`-core`) is reachable via path dep.
//! 2. `[workspace.dependencies]` inheritance flows through to a member
//!    bin's runtime -- the binary calls into core which calls into
//!    scalo.

use ci_test_rust_workspace_core::{banner, setup_logger};

fn main() {
    // setup_logger lives in the core crate; if workspace dep
    // inheritance is broken, this won't compile.
    let _ = setup_logger();

    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--version") {
        println!("ci-test-rust-workspace v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    println!("{}", banner());
    println!("ci-test-rust-workspace-cli v{}", env!("CARGO_PKG_VERSION"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banner_reachable_via_core() {
        let s = banner();
        assert!(s.contains("ci-test-rust-workspace-core"));
    }
}
