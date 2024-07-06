use image::{codecs::tga::TgaDecoder, DynamicImage, RgbaImage};
use image_blp::{convert::blp_to_image, parser::load_blp_from_buf};

use crate::extractor::W3Raw;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageType {
    BLP,
    TGA,
}

pub struct ImageRaw {
    pub data: Vec<u8>,
    pub image_type: ImageType,
}

impl ImageRaw {
    pub fn new(data: Vec<u8>, image_type: ImageType) -> Self {
        ImageRaw { data, image_type }
    }

    pub fn from_w3raw(w3raw: W3Raw) -> Self {
        let file_path = std::path::Path::new(&w3raw.filename);
        let image_type = match file_path.extension() {
            Some(ext) => match ext.to_str().unwrap() {
                "blp" => ImageType::BLP,
                "tga" => ImageType::TGA,
                _ => panic!("Unsupported image type"),
            },
            None => panic!("Unsupported image type"),
        };

        Self::new(w3raw.data, image_type)
    }
}

impl TryInto<RgbaImage> for ImageRaw {
    type Error = &'static str;

    fn try_into(self) -> Result<RgbaImage, Self::Error> {
        if self.image_type == ImageType::BLP {
            match load_blp_from_buf(&self.data) {
                Ok(blp) => match blp_to_image(&blp, 0) {
                    Ok(img) => Ok(img.to_rgba8()),
                    Err(_) => Err("Failed to convert BLP image"),
                },
                Err(_) => Err("Failed to load BLP image"),
            }
        } else {
            let csr = std::io::Cursor::new(&self.data);
            match TgaDecoder::new(csr) {
                Ok(decoder) => match DynamicImage::from_decoder(decoder) {
                    Ok(img) => Ok(img.to_rgba8()),
                    Err(_) => Err("Failed to convert TGA image"),
                },
                Err(_) => Err("Failed to load TGA image"),
            }
        }
    }
}
