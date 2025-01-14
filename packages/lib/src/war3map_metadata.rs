use binary_reader::{BinaryReader, Endian};

use crate::parser::{
    binary_reader::BinaryReadable,
    imp::War3MapImp,
    w3i::War3MapW3i,
    w3x::{Image, War3MapW3x},
    wts::War3MapWts,
};

pub struct War3MapMetadata {
    pub map_info: Option<War3MapW3i>,
    pub imp: Option<War3MapImp>,
    pub wts: Option<War3MapWts>,
    pub minimap: Option<Image>,
    pub preview: Option<Image>,
}

impl War3MapMetadata {
    pub fn from(buffer: &[u8]) -> Option<Self> {
        let mut binary_reader = BinaryReader::from_u8(buffer);
        binary_reader.set_endian(Endian::Little);

        if let Ok(w3x) = War3MapW3x::load(&mut binary_reader, 0) {
            let mut w3x_box = Box::new(w3x);
            Some(Self {
                map_info: w3x_box.read_map_info().ok(),
                imp: w3x_box.read_imports().ok(),
                wts: w3x_box.read_string_table().ok(),
                minimap: w3x_box.read_minimap().ok(),
                preview: w3x_box.read_preview().ok(),
            })
        } else {
            None
        }
    }
}
