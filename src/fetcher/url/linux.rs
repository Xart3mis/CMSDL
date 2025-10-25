pub fn path(version: &str) -> Option<String> {
    #[cfg(target_arch = "aarch64")]
    return Some(format!("geckodriver-{}-linux-aarch64.tar.gz", version));

    #[cfg(target_arch = "x86")]
    return Some(format!("geckodriver-{}-linux32.tar.gz", version));

    #[cfg(target_arch = "x86_64")]
    return Some(format!("geckodriver-{}-linux64.tar.gz", version));

    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64")))]
    return None;
}
