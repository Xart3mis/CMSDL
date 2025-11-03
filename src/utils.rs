use std::path::{Path, PathBuf};

pub fn is_valid_path(path_str: &str) -> bool {
    if path_str.contains('\0') {
        return false;
    }

    let path = Path::new(path_str);

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
