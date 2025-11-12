use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error(transparent)]
    StyleTemplateError(#[from] indicatif::style::TemplateError),

    #[error(transparent)]
    CurlMultiError(#[from] curl::MultiError),

    #[error(transparent)]
    CurlError(#[from] curl::Error),

    #[error(transparent)]
    FileError(#[from] std::io::Error),
}
