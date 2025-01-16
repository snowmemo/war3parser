use image::{codecs::tga::TgaDecoder, DynamicImage, RgbaImage};

use super::error::ParserError;

/// TGA image
pub struct TgaImage {
    pub width: u32,
    pub height: u32,
    pub data: RgbaImage,
}

impl TgaImage {
    /// Load a TGA image from a buffer
    pub fn load<T: AsRef<[u8]>>(buffer: &T) -> Result<Self, ParserError> {
        let cursor = std::io::Cursor::new(buffer);
        let decoder = TgaDecoder::new(cursor)?;
        let image = DynamicImage::from_decoder(decoder)?;

        Ok(Self {
            width: image.width(),
            height: image.height(),
            data: image.to_rgba8(),
        })
    }
}
