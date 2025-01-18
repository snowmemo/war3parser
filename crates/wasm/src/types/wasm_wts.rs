use js_sys;
use war3parser::parser::wts::War3MapWts;
use wasm_bindgen::prelude::wasm_bindgen;

use super::utils::hashmap_to_js_map;

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct WasmWts {
    pub string_map: js_sys::Map,
}

impl From<War3MapWts> for WasmWts {
    fn from(wts: War3MapWts) -> Self {
        Self {
            string_map: hashmap_to_js_map(wts.string_map),
        }
    }
}
