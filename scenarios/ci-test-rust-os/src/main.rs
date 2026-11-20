// Project:   hyperi-ci-test / ci-test-rust-os
// License:   BUSL-1.1
// Copyright: (c) 2026 HYPERI PTY LIMITED

//! Prints the platform report each OS crate builds.

// hyperi-ci passes jemalloc everywhere but msvc, and mimalloc on msvc when it
// is declared. Quality runs clippy --all-features, so jemalloc wins when both
// are on.
#[cfg(all(feature = "jemalloc", not(target_env = "msvc")))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[cfg(all(
    feature = "mimalloc",
    not(all(feature = "jemalloc", not(target_env = "msvc")))
))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    if std::env::args().any(|a| a == "--version") {
        println!("ci-test-rust-os {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    println!("{}", report());
}

fn report() -> String {
    format!("{} | {}", os_core::compressed_roundtrip(), platform())
}

#[cfg(windows)]
fn platform() -> String {
    os_windows::describe()
}

#[cfg(target_os = "linux")]
fn platform() -> String {
    os_linux::describe()
}

#[cfg(target_os = "macos")]
fn platform() -> String {
    os_macos::describe()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_names_this_os() {
        assert!(report().contains(std::env::consts::OS));
    }
}
