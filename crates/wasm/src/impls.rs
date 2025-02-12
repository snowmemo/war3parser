use war3parser::{parser::War3MapW3i, war3map_metadata::War3MapMetadata as War3MapMetadataOri};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{War3Image, War3MapMetadata};

/// Extracts map information from a War3 map file buffer.
///
/// # Arguments
/// * `buffer` - A JavaScript `Uint8Array` containing the map file data
///
/// # Returns
/// * `Option<War3MapMetadata>` - The parsed map metadata if successful, including:
///   - Map information (w3i)
///   - Image resources
#[wasm_bindgen]
pub fn get_map_info(buffer: js_sys::Uint8Array) -> Option<War3MapMetadata> {
    let buffer_vec = buffer.to_vec();
    match War3MapMetadataOri::from(&buffer_vec) {
        Some(mut metadata) => {
            metadata.update_string_table().ok();
            let map_info = metadata.map_info.map(War3MapW3i::from);
            let images: Vec<War3Image> = metadata.images.iter().map(War3Image::from).collect();
            Some(War3MapMetadata { map_info, images })
        }
        None => None,
    }
}
