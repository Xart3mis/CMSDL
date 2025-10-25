pub fn path(version: &str) -> Option<String> {
    #[cfg(target_arch = "aarch64")]
    return Some(format!("geckodriver-{}-win-aarch64.zip", version));

    #[cfg(target_arch = "x86")]
    return Some(format!("geckodriver-{}-win32.zip", version));

    #[cfg(target_arch = "x86_64")]
    return Some(format!("geckodriver-{}-win64.zip", version));

    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64")))]
    return None;
}
