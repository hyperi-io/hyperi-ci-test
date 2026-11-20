//! macOS-only: compiles to nothing elsewhere.
#![cfg(target_os = "macos")]

/// Name the OS and the CPU arch.
pub fn describe() -> String {
    format!("{} {}", std::env::consts::OS, std::env::consts::ARCH)
}

#[cfg(test)]
mod tests {
    #[test]
    fn names_macos() {
        assert!(super::describe().starts_with("macos "));
    }
}
