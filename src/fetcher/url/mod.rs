mod linux;
mod macos;
mod windows;

use reqwest::blocking::Client;
use serde_json::Value;

#[cfg(target_os = "windows")]
use crate::fetcher::url::windows as os;

#[cfg(target_os = "linux")]
use crate::fetcher::url::linux as os;

#[cfg(target_os = "macos")]
use crate::fetcher::url::macos as os;

fn filename_builder(version: String) -> Option<String> {
    if let Some(p) = os::path(&version) {
        return Some(p);
    }

    None
}

fn get_driver_version() -> Option<String> {
    if let Ok(res) = Client::builder()
        .user_agent("CMSDL/1.0")
        .build()
        .unwrap()
        .get("https://api.github.com/repos/mozilla/geckodriver/releases/latest")
        .send()
        && let Ok(text) = res.text()
        && let Ok(json) = serde_json::from_str::<Value>(&text)
        && let Some(tag) = json["tag_name"].as_str()
    {
        return Some(tag.to_string());
    }

    None
}

pub fn filename() -> Option<String> {
    filename_builder(get_driver_version()?)
}

pub fn get_url() -> Option<String> {
    Some(format!(
        "https://github.com/mozilla/geckodriver/releases/download/{}/{}",
        get_driver_version()?,
        filename()?
    ))
}


