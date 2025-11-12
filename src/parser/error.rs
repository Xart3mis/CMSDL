use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("failed to parse regex expression")]
    RegexError(#[from] regex::Error),

    #[error("failed to parse css selector: {0}")]
    SelectorError(String),

    #[error("error with curl client: {0}")]
    ClientError(#[from] crate::client::error::ClientError),

    #[error("failed to parse ID")]
    ParseIntError(#[from] std::num::ParseIntError),
}

impl<'a> From<scraper::error::SelectorErrorKind<'a>> for ParseError {
    fn from(err: scraper::error::SelectorErrorKind<'a>) -> Self {
        ParseError::SelectorError(err.to_string())
    }
}
