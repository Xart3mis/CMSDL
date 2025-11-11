use anyhow::Result;
use dialoguer::{Input, Password, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

use super::utils::is_valid_path;

const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl Credentials {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    pub fn prompt() -> Result<Credentials> {
        let username: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Username")
            .interact_text()?;

        let password: String = Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Password")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.chars().count() >= 8 {
                    Ok(())
                } else {
                    Err("Password must be longer than 8 characters long.")
                }
            })
            .interact()?;

        Ok(Self::new(&username, &password))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadOptions {
    pub max_concurrency: Option<usize>,
    pub max_file_size: Option<usize>,
    pub save_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GeneralOptions {
    pub interactive_filtering: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub general_options: GeneralOptions,
    pub credentials: Credentials,
    pub download_options: DownloadOptions,
}

impl Config {
    pub fn load() -> Result<Self> {
        if Path::new(CONFIG_FILE).exists() {
            Self::load_from_file()
        } else {
            println!("Config file not found. Creating new configuration...");
            Self::create_new()
        }
    }

    fn load_from_file() -> Result<Self> {
        let content = fs::read_to_string(CONFIG_FILE)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    fn create_new() -> Result<Self> {
        let credentials = Credentials::prompt()?;
        let save_path = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Save Downloads To")
            .validate_with(|input: &String| -> Result<(), &str> {
                if is_valid_path(input) {
                    Ok(())
                } else {
                    Err("Invalid path")
                }
            })
            .interact_text()?
            .into();

        let download_options = DownloadOptions {
            max_concurrency: None,
            max_file_size: None,
            save_path,
        };

        let config = Config {
            general_options: GeneralOptions::default(),
            download_options,
            credentials,
        };

        config.save()?;
        println!("Configuration saved to {}", CONFIG_FILE);
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(CONFIG_FILE, toml_string)?;
        Ok(())
    }

    /// Update credentials and save
    pub fn update_credentials(&mut self, username: String, password: String) -> Result<()> {
        self.credentials = Credentials { username, password };
        self.save()
    }
}
