use image::RgbaImage;
use image_blp::{convert::blp_to_image, parser::load_blp_from_buf};

use super::error::ParserError;

/// BLP image
pub struct BlpImage {
    pub width: u32,
    pub height: u32,
    pub data: RgbaImage,
}

impl BlpImage {
    /// Load a BLP image from a buffer
    pub fn load<T: AsRef<[u8]>>(buffer: &T) -> Result<Self, ParserError> {
        let blp = load_blp_from_buf(buffer.as_ref())?;
        let image = blp_to_image(&blp, 0)?;
        Ok(Self {
            width: image.width(),
            height: image.height(),
            data: image.to_rgba8(),
        })
    }
}
