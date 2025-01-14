use binary_reader::BinaryReader;

use crate::parser::binary_reader::{AutoReadable, BinaryReadable};
use anyhow::Result;

pub struct RandomItem {
    pub chance: i32,
    pub id: [u8; 4],
}

pub struct RandomItemSet {
    pub items: Vec<RandomItem>,
}

pub struct RandomItemTable {
    pub id: i32,
    pub name: String,
    pub sets: Vec<RandomItemSet>,
}

impl BinaryReadable for RandomItem {
    fn load(stream: &mut BinaryReader, _version: u32) -> Result<Self> {
        Ok(Self {
            chance: AutoReadable::read(stream)?,
            id: AutoReadable::read(stream)?,
        })
    }
}

impl BinaryReadable for RandomItemSet {
    fn load(stream: &mut BinaryReader, _version: u32) -> Result<Self> {
        Ok(Self {
            items: {
                let count: u32 = AutoReadable::read(stream)?;
                let mut items: Vec<RandomItem> = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    items.push(RandomItem::load(stream, _version)?);
                }
                items
            },
        })
    }
}

impl BinaryReadable for RandomItemTable {
    fn load(stream: &mut BinaryReader, _version: u32) -> Result<Self> {
        Ok(Self {
            id: AutoReadable::read(stream)?,
            name: AutoReadable::read(stream)?,
            sets: {
                let count: u32 = AutoReadable::read(stream)?;
                let mut sets: Vec<RandomItemSet> = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    sets.push(RandomItemSet::load(stream, _version)?);
                }
                sets
            },
        })
    }
}
