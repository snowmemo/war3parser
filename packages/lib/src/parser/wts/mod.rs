use anyhow::Result;
use regex::Regex;
use wasm_bindgen::JsValue;

use crate::extractor::W3Raw;

/// TRIGSTR_007 / TRIGSTR_007ab / TRIGSTR_7 TRIGSTR_-007
pub const TRAGGER_STR_RE: &str = r"TRIGSTR_(-?\d+)(?:\w+)?";

/// STRING_007
pub const STRINGS_RE: &str = r"STRING\s+([0-9]+)\s+\{\r\n+([^\}]*)\r\n\}";

pub struct War3MapWts {
    pub string_map: js_sys::Map,
}

impl War3MapWts {
    pub fn load(buffer: &Vec<u8>) -> Result<Self> {
        let buffer = String::from_utf8_lossy(buffer).to_string();
        let re = Regex::new(STRINGS_RE).unwrap();
        let string_map = js_sys::Map::new();
        for caps in re.captures_iter(buffer.as_str()) {
            let id = caps.get(1).unwrap().as_str().to_string();
            let content = String::from(caps.get(2).unwrap().as_str());
            if let Ok(id) = id.parse::<i32>() {
                string_map.set(&JsValue::from(id), &JsValue::from(content));
            }
        }
        Ok(Self { string_map })
    }

    pub fn try_from(w3raw: W3Raw) -> Result<Self> {
        Self::load(&w3raw.data)
    }
}
