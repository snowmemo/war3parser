use crate::parser::{
    error::ParserError,
    imp::War3MapImp,
    w3i::War3MapW3i,
    w3x::{War3Image, War3MapW3x},
    wts::War3MapWts,
};

pub struct War3MapMetadata {
    pub map_info: Option<War3MapW3i>,
    pub imp: Option<War3MapImp>,
    pub wts: Option<War3MapWts>,
    pub images: Vec<War3Image>,
}

impl War3MapMetadata {
    pub fn from(buffer: &[u8]) -> Option<Self> {
        if let Ok(w3x) = War3MapW3x::from_buffer(buffer) {
            let mut w3x_box = Box::new(w3x);
            let mut images = Vec::new();
            if let Ok(minimap) = w3x_box.read_minimap() {
                images.push(minimap);
            }
            if let Ok(preview) = w3x_box.read_preview() {
                images.push(preview);
            }

            Some(Self {
                map_info: w3x_box.read_map_info().ok(),
                imp: w3x_box.read_imports().ok(),
                wts: w3x_box.read_string_table().ok(),
                images,
            })
        } else {
            None
        }
    }

    pub fn update_string_table(&mut self) -> Result<(), ParserError> {
        let map_info = self
            .map_info
            .as_ref()
            .ok_or(ParserError::MapFileNotFound("w3i".to_string()))?;
        let mut map_info_json = serde_json::to_string(map_info)?;
        let trigger_string_map = map_info.trigger_string_map()?;

        let string_table = &self
            .wts
            .as_ref()
            .ok_or(ParserError::MapFileNotFound("wts".to_string()))?
            .string_map;
        let default = String::new();

        trigger_string_map.iter().for_each(|(key, value)| {
            let replace_str = string_table.get(value).unwrap_or(&default);
            map_info_json = map_info_json.replace(key, replace_str);
        });

        Ok(())
    }
}
