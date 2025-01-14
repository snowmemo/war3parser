use binary_reader::BinaryReader;
use wasm_bindgen::JsValue;

use super::{
    binary_reader::{AutoReadable, BinaryReadable},
    error::ParserError,
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Import {
    pub is_custom: u8,
    pub path: String,
}

#[derive(Default)]
pub struct War3MapImp {
    pub version: u32,
    pub entries: js_sys::Map,
}

impl BinaryReadable for Import {
    fn load(stream: &mut BinaryReader, _version: u32) -> Result<Self, ParserError> {
        Ok(Import {
            is_custom: AutoReadable::read(stream)?,
            path: AutoReadable::read(stream)?,
        })
    }
}

impl BinaryReadable for War3MapImp {
    fn load(stream: &mut BinaryReader, _version: u32) -> Result<Self, ParserError> {
        let version: u32 = AutoReadable::read(stream)?;
        let count: u32 = AutoReadable::read(stream)?;
        let entries = js_sys::Map::new();
        for _ in 0..count {
            let import = Import::load(stream, version)?;
            if import.is_custom > 1 {
                entries.set(
                    &JsValue::from(&import.path),
                    &serde_wasm_bindgen::to_value(&import)?,
                );
            } else {
                entries.set(
                    &JsValue::from(format!("war3mapimported\\{}", import.path)),
                    &serde_wasm_bindgen::to_value(&import)?,
                );
            }
        }
        Ok(War3MapImp { version, entries })
    }
}
