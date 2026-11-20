// Project:   ci-test-rust-workspace-core
// License:   FSL-1.1-ALv2
// Copyright: (c) 2026 HYPERI PTY LIMITED

//! Core lib for the workspace test -- proves that scalo's
//! ``[workspace.dependencies]`` inheritance + member-level
//! ``.workspace = true`` consumption resolves and links cleanly.

/// Initialise scalo's default logger. Returns whatever scalo returns.
///
/// Used by the cli member to verify the inherited workspace dep is
/// reachable across crate boundaries -- a member library exposes a
/// thin wrapper, the cli binary calls it. If the inheritance breaks,
/// this fails to compile at the cli member.
pub fn setup_logger() -> Result<(), scalo::logger::LoggerError> {
    scalo::logger::setup_default()
}

/// Return a single-line banner for the workspace.
///
/// Includes the workspace package version pulled from
/// ``CARGO_PKG_VERSION`` -- exercises the workspace.package.version
/// inheritance as well.
pub fn banner() -> String {
    format!("ci-test-rust-workspace-core v{}", env!("CARGO_PKG_VERSION"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banner_includes_version() {
        let s = banner();
        assert!(s.contains(env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn setup_logger_callable() {
        // Don't unwrap — sibling tests in the same process may have
        // already initialised the logger.
        let _ = setup_logger();
    }
}
