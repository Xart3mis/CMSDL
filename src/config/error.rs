use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to read user input")]
    Input(#[from] dialoguer::Error),

    #[error("serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("deserialization error: {0}")]
    TomlDeser(#[from] toml::de::Error),

    #[error("I/O error")]
    IO(#[from] std::io::Error),

    #[error("Invalid Path")]
    InvalidPath,
}
