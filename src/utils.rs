use std::path::{Path, PathBuf};

pub fn is_valid_path(path_str: &str) -> bool {
    if path_str.contains('\0') {
        return false;
    }

    let path = Path::new(path_str);

    #[cfg(windows)]
    {
        const INVALID_CHARS: &[char] = &['<', '>', ':', '"', '|', '?', '*'];
        if path_str.chars().any(|c| INVALID_CHARS.contains(&c)) {
            return false;
        }

        let reserved = [
            "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7",
            "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
        ];

        for comp in path.components() {
            if let Some(name) = comp.as_os_str().to_str() {
                let upper = name.trim_end_matches('.').to_ascii_uppercase();
                if reserved.contains(&upper.as_str()) {
                    return false;
                }
            }
        }
    }

    let normalized: PathBuf = if path.is_absolute() {
        path.to_path_buf()
    } else {
        match std::env::current_dir() {
            Ok(cwd) => cwd.join(path),
            Err(_) => return false,
        }
    };

    if normalized.as_os_str().is_empty() {
        return false;
    }

    true
}
