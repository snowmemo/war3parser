use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Failed to find STRINGS")]
    FailedToFindStrings,
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("From UTF-8 error")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}
