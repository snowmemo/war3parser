use image::{codecs::tga::TgaDecoder, DynamicImage};
use nom::FindSubstring;
use photon_rs::{to_image_data, PhotonImage};

use super::error::ParserError;
use web_sys::ImageData;

pub struct TgaImage {
    pub width: u32,
    pub height: u32,
    pub data: ImageData,
}

impl TgaImage {
    pub fn load<T: AsRef<[u8]>>(buffer: &T) -> Result<Self, ParserError> {
        let cursor = std::io::Cursor::new(buffer);
        let decoder = TgaDecoder::new(cursor)?;
        let image = DynamicImage::from_decoder(decoder)?;

        let proton = PhotonImage::new(image.to_rgba8().to_vec(), image.width(), image.height());
        let data = to_image_data(proton);
        Ok(Self {
            width: image.width(),
            height: image.height(),
            data,
        })
    }

    pub fn is_tga<T: AsRef<[u8]> + Sized>(buffer: &T) -> bool {
        let bytes = buffer.as_ref();
        bytes.find_substring("TRUEVISION-XFILE.\0").is_some()
    }
}
