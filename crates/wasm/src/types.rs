use war3parser::parser::w3i::War3MapW3i;
use war3parser::prelude::{War3Image as War3ImageOri, War3ImageBase64};

/// Preview and minimap images
#[derive(Debug, tsify_next::Tsify, serde::Serialize, serde::Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct War3Image {
    pub data_url: String,
    pub width: u32,
    pub height: u32,
    pub filename: String,
}
impl From<&War3ImageOri> for War3Image {
    fn from(image: &War3ImageOri) -> Self {
        let width = image.data.width();
        let height = image.data.height();
        let war3image_base64 = War3ImageBase64::try_from((*image).clone()).unwrap();
        Self {
            data_url: war3image_base64.data,
            width,
            height,
            filename: war3image_base64.filename,
        }
    }
}
// Full information for wasm
#[derive(Debug, tsify_next::Tsify, serde::Serialize, serde::Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct War3MapMetadata {
    pub map_info: Option<War3MapW3i>,
    pub images: Vec<War3Image>,
}
