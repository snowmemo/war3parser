use binary_reader::BinaryReader;

use crate::parser::{
    binary_reader::{AutoReadable, BinaryReadable},
    error::ParserError,
};

pub struct UpgradeAvailabilityChange {
    pub player_flags: u32,
    pub id: [u8; 4],
    pub level_affected: i32,
    pub availability: i32,
}

impl BinaryReadable for UpgradeAvailabilityChange {
    fn load(stream: &mut BinaryReader, _version: u32) -> Result<Self, ParserError> {
        Ok(Self {
            player_flags: AutoReadable::read(stream)?,
            id: AutoReadable::read(stream)?,
            level_affected: AutoReadable::read(stream)?,
            availability: AutoReadable::read(stream)?,
        })
    }
}
