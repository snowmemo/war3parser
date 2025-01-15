use thiserror::Error;

/// Custom error types
#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Failed to find STRINGS")]
    FailedToFindStrings,
    #[error("Failed to find minimap")]
    FailedToFindMinimap,
    #[error("Map file {0} not found")]
    MapFileNotFound(String),
    #[error("Failed to serialize map info")]
    FailedToSerializeMapInfo,
    #[error("Failed to load image")]
    FailedToLoadImage(#[from] image::ImageError),
    #[error("Failed to load BLP image")]
    FailedToLoadBLP(#[from] image_blp::parser::LoadError),
    #[error("Failed to convert BLP image")]
    FailedToConvert(#[from] image_blp::convert::Error),
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("Regex error")]
    RegexError(#[from] regex::Error),
    #[error("From UTF-8 error")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("Failed to serialize")]
    FailedToSerialize(#[from] serde_json::Error),
}
