use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Failed to find STRINGS")]
    FailedToFindStrings,
    #[error("Failed to find END")]
    FailedToFindEnd,
}
