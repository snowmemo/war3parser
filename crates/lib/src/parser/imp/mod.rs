use std::collections::HashMap;

use binary_reader::BinaryReader;

use super::{
    binary_reader::{AutoReadable, BinaryReadable},
    error::ParserError,
};

/// Import entry
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct Import {
    pub is_custom: u8,
    pub path: String,
}

/// Import table
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default)]
pub struct War3MapImp {
    pub version: u32,
    pub entries: HashMap<String, Import>,
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
        let mut entries = HashMap::new();
        for _ in 0..count {
            let import = Import::load(stream, version)?;
            if import.is_custom > 1 {
                entries.insert(import.path.clone(), import);
            } else {
                entries.insert(format!("war3mapimported\\{}", import.path.clone()), import);
            }
        }
        Ok(War3MapImp { version, entries })
    }
}
