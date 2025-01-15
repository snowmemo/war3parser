use binary_reader::BinaryReader;
use serde::{Deserialize, Serialize};

use crate::parser::{
    binary_reader::{AutoReadable, BinaryReadable},
    error::ParserError,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct TechAvailabilityChange {
    pub player_flags: u32,
    pub id: [u8; 4],
}

impl BinaryReadable for TechAvailabilityChange {
    fn load(stream: &mut BinaryReader, _version: u32) -> Result<Self, ParserError> {
        Ok(Self {
            player_flags: AutoReadable::read(stream)?,
            id: AutoReadable::read(stream)?,
        })
    }
}
