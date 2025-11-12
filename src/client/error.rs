use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Unexpected curl error: {0}")]
    CurlError(#[from] curl::Error),

    #[error("failed to build string from utf8")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}
