#![allow(dead_code)]

use war3parser::{parser::w3x::War3MapW3x, war3map_metadata::War3MapMetadata};
use wasm_bindgen_test::*;

fn load_map() -> &'static [u8] {
    include_bytes!("../../../test_data/Legion_TD_11.1c_TeamOZE.w3x")
}

#[wasm_bindgen_test]
fn test_w3x_parse() {
    let map = load_map();
    let _w3x = War3MapW3x::from_buffer(map).expect("failed to parse w3x");
}

#[wasm_bindgen_test]
fn test_wasm_mapinfo() {
    let map = load_map();
    let map_info = War3MapMetadata::from(map).expect("failed to parse map info");
    assert!(map_info.map_info.is_some());
}
