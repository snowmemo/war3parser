use image::{codecs::tga::TgaDecoder, DynamicImage, RgbaImage};

use super::error::ParserError;
pub struct TgaImage {
    pub width: u32,
    pub height: u32,
    pub data: RgbaImage,
}

impl TgaImage {
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

    pub fn is_tga<T: AsRef<[u8]> + Sized>(buffer: &T) -> bool {
        let bytes = buffer.as_ref();
        let pattern = b"TRUEVISION-XFILE.\0";
        bytes
            .windows(pattern.len())
            .position(|window| window == pattern)
            .is_some()
    }
}
