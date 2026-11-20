// Project:   ci-test-rust-lib
// License:   FSL-1.1-ALv2
// Copyright: (c) 2026 HYPERI PTY LIMITED

//! Rust library variant for the hyperi-ci test matrix.
//!
//! This crate has no binary target — the container detector
//! classifies it as library-only and skips the container build stage
//! entirely. Publish target is crates.io only.
//!
//! The only public surface is a thin re-export of scalo's logger.
//! Enough to validate the dep resolves and links correctly when the
//! lib is consumed from another crate.

pub use scalo::logger;

/// Crate version from Cargo.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialise scalo's default logger.
///
/// # Errors
///
/// Returns whatever scalo returns from setup_default -- typically a
/// LoggerError if the global recorder is already installed.
pub fn setup_logger() -> Result<(), logger::LoggerError> {
    logger::setup_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_non_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn setup_logger_is_callable() {
        // Don't unwrap — sibling tests may already have initialised.
        let _ = setup_logger();
    }
}
