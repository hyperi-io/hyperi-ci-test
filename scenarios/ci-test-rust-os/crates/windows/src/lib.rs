//! Windows-only: compiles to nothing elsewhere, so only a Windows leg lints it.
#![cfg(windows)]

/// Name the OS and the milliseconds since boot, from the Win32 API.
pub fn describe() -> String {
    // SAFETY: GetTickCount64 takes no arguments and cannot fail.
    let uptime = unsafe { windows::Win32::System::SystemInformation::GetTickCount64() };
    format!("{} up {uptime} ms", std::env::consts::OS)
}

#[cfg(test)]
mod tests {
    #[test]
    fn names_windows() {
        assert!(super::describe().starts_with("windows up "));
    }
}
