use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error parsing style template")]
    StyleTemplate(#[from] indicatif::style::TemplateError),

    #[error(transparent)]
    CurlMulti(#[from] curl::MultiError),

    #[error(transparent)]
    Curl(#[from] curl::Error),

    #[error("I/O error occurred")]
    File(#[from] std::io::Error),
}
