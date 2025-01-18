use war3parser::prelude::{War3Image, War3ImageBase64};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct WasmImage {
    pub data_url: String,
    pub filename: String,
}

impl From<War3Image> for WasmImage {
    fn from(image: War3Image) -> Self {
        let war3image_base64 = War3ImageBase64::try_from(image).unwrap();
        Self {
            data_url: war3image_base64.data,
            filename: war3image_base64.filename,
        }
    }
}
