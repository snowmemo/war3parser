use image_blp::{convert::blp_to_image, parser::load_blp_from_buf};
use photon_rs::{to_image_data, PhotonImage};

use super::error::ParserError;
use web_sys::ImageData;

pub struct BlpImage {
    pub width: u32,
    pub height: u32,
    pub data: ImageData,
}

impl BlpImage {
    pub fn load<T: AsRef<[u8]>>(buffer: &T) -> Result<Self, ParserError> {
        let blp = load_blp_from_buf(buffer.as_ref())?;
        let image = blp_to_image(&blp, 0)?;
        let proton = PhotonImage::new(image.to_rgba8().to_vec(), image.width(), image.height());
        let data = to_image_data(proton);
        Ok(Self {
            width: image.width(),
            height: image.height(),
            data,
        })
    }

    pub fn is_blp<T: AsRef<[u8]> + Sized>(buffer: &T) -> bool {
        let bytes = buffer.as_ref();
        bytes[0] == 0x42 && bytes[1] == 0x4C && bytes[2] == 0x50 && bytes[3] == 0x30
    }
}
