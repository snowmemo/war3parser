use regex::Regex;
use wasm_bindgen::JsValue;

use crate::extractor::W3Raw;

use super::error::ParserError;

/// TRIGSTR_007 / TRIGSTR_007ab / TRIGSTR_7 TRIGSTR_-007
pub const TRAGGER_STR_RE: &str = r"TRIGSTR_(-?\d+)(?:\w+)?";

/// STRING_007
pub const STRINGS_RE: &str = r"STRING\s+([0-9]+)\s+\{\r\n+([^\}]*)\r\n\}";

pub struct War3MapWts {
    pub string_map: js_sys::Map,
}

impl War3MapWts {
    pub fn load(buffer: &String) -> Result<Self, ParserError> {
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
}
