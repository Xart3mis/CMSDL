pub mod url;
use url::get_url;

use flate2::read::GzDecoder;
use tar::Archive;

use zip::ZipArchive;

use std::{
    env, io,
    io::Cursor,
    path::{Path, PathBuf},
};

use reqwest::blocking;

fn expand_relative_path(relative_path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = relative_path.as_ref();

    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        let current_dir = env::current_dir()?;
        let absolute_path = current_dir.join(path);

        Ok(absolute_path)
    }
}

#[cfg(target_os = "windows")]
pub fn fetch_geckodriver() -> Option<PathBuf> {
    if let Some(url) = get_url()
        && let Ok(response) = blocking::get(url)
    {
        let archive = ZipArchive::new(Cursor::new(response.bytes().unwrap()));
        archive.unwrap().extract(".").unwrap();

        if let Ok(path) = expand_relative_path("geckodriver.exe") {
            return Some(path);
        }
    }

    None
}

#[cfg(all(any(target_os = "linux", target_os = "macos"), unix))]
pub fn fetch_geckodriver() -> Option<PathBuf> {
    if let Some(url) = get_url()
        && let Ok(response) = blocking::get(url)
    {
        let gz_decoder = GzDecoder::new(response);
        let mut archive = Archive::new(gz_decoder);

        archive.unpack(".").unwrap();

        use std::fs::{Permissions, set_permissions};
        use std::os::unix::fs::PermissionsExt;

        set_permissions("geckodriver", Permissions::from_mode(0o775)).unwrap();

        if let Ok(path) = expand_relative_path("geckodriver") {
            return Some(path);
        }
    }

    None
}
