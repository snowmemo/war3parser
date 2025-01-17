use war3parser::parser::w3x::War3Image;
use wasm_bindgen::{prelude::wasm_bindgen, Clamped};

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct WasmImage {
    pub width: u32,
    pub height: u32,
    pub data: web_sys::ImageData,
    pub filename: String,
}

impl From<War3Image> for WasmImage {
    fn from(image: War3Image) -> Self {
        let data = image.data.clone().to_vec();
        let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&data),
            image.width,
            image.height,
        )
        .unwrap();
        Self {
            width: image.width,
            height: image.height,
            data: image_data,
            filename: image.filename,
        }
    }
}
