//! Linux-only: compiles to nothing elsewhere.
#![cfg(target_os = "linux")]

/// Name the OS and the kernel release from /proc.
pub fn describe() -> String {
    let release = std::fs::read_to_string("/proc/sys/kernel/osrelease").unwrap_or_default();
    format!("{} {}", std::env::consts::OS, release.trim())
}

#[cfg(test)]
mod tests {
    #[test]
    fn names_linux_and_its_kernel() {
        let text = super::describe();
        assert!(text.starts_with("linux "));
        assert!(text.len() > "linux ".len());
    }
}
