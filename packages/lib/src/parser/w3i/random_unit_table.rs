use binary_reader::BinaryReader;

use crate::parser::{
    binary_reader::{AutoReadable, BinaryReadable},
    error::ParserError,
};

#[derive(Debug)]
pub struct RandomUnit {
    pub chance: i32,
    pub ids: Vec<[u8; 4]>,
}

#[derive(Debug)]
pub struct RandomUnitTable {
    pub id: i32,
    pub name: String,
    pub position: i32,
    pub column_types: Vec<i32>,
    pub units: Vec<RandomUnit>,
}

impl BinaryReadable for RandomUnit {
    fn load(stream: &mut BinaryReader, position: u32) -> Result<Self, ParserError> {
        Ok(Self {
            chance: AutoReadable::read(stream)?,
            ids: {
                let mut ids: Vec<[u8; 4]> = Vec::with_capacity(position as usize);
                for _ in 0..position {
                    ids.push(AutoReadable::read(stream)?);
                }
                ids
            },
        })
    }
}

impl BinaryReadable for RandomUnitTable {
    fn load(stream: &mut BinaryReader, _version: u32) -> Result<Self, ParserError> {
        let id: i32 = AutoReadable::read(stream)?;
        let name: String = AutoReadable::read(stream)?;
        let position: i32 = AutoReadable::read(stream)?;
        let mut column_types: Vec<i32> = Vec::with_capacity(position as usize);
        for _ in 0..position {
            column_types.push(AutoReadable::read(stream)?);
        }
        let mut units: Vec<RandomUnit> = Vec::with_capacity(position as usize);
        for _ in 0..position {
            units.push(RandomUnit::load(stream, position as u32)?);
        }
        Ok(Self {
            id,
            name,
            position,
            column_types,
            units,
        })
    }
}
