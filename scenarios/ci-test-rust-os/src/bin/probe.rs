// Project:   hyperi-ci-test / ci-test-rust-os
// License:   BUSL-1.1
// Copyright: (c) 2026 HYPERI PTY LIMITED

//! A second binary, so packaging names more than one per target.

fn main() {
    if std::env::args().any(|a| a == "--version") {
        println!("ci-test-rust-os-probe {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    println!("{}", std::env::consts::OS);
}
