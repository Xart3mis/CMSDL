use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Curl(#[from] curl::Error),

    #[error("failed to build string from utf8")]
    FromUtf8(#[from] std::string::FromUtf8Error),
}
