use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to parse regex expression")]
    Regex(#[from] regex::Error),

    #[error("failed to parse css selector: {0}")]
    Selector(String),

    #[error("error with curl client: {0}")]
    Client(#[from] crate::client::error::Error),

    #[error("failed to parse ID: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}

impl<'a> From<scraper::error::SelectorErrorKind<'a>> for Error {
    fn from(err: scraper::error::SelectorErrorKind<'a>) -> Self {
        Error::Selector(err.to_string())
    }
}
