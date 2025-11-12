use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("failed to read user input")]
    InputError(#[from] dialoguer::Error),

    #[error("serialization error: {0}")]
    TomlSerError(#[from] toml::ser::Error),

    #[error("deserialization error: {0}")]
    TomlDeserError(#[from] toml::de::Error),

    #[error("I/O error")]
    IOError(#[from] std::io::Error),
}
