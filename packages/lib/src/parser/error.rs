use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Failed to find STRINGS")]
    FailedToFindStrings,
    #[error("Failed to find minimap")]
    FailedToFindMinimap,
    #[error("Failed to parse")]
    FailedToParse(#[from] serde_wasm_bindgen::Error),
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
}

