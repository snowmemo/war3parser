use photon_rs::{to_image_data, PhotonImage};
use war3parser::parser::w3x::War3Image;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct WasmImage {
    pub width: u32,
    pub height: u32,
    pub data: web_sys::ImageData,
}

impl From<War3Image> for WasmImage {
    fn from(image: War3Image) -> Self {
        let data = image.data.clone().to_vec();
        let photon_image = PhotonImage::new(data, image.width, image.height);
        let image_data = to_image_data(photon_image);
        Self {
            width: image.width,
            height: image.height,
            data: image_data,
        }
    }
}
