pub mod force;
pub mod player;
pub mod random_item_table;
pub mod random_unit_table;
pub mod tech_availability_change;
pub mod upgrade_availability_change;

use binary_reader::BinaryReader;

use crate::{
    extractor::W3Raw,
    parser::binary_reader::{AutoReadable, BinaryReadable},
};

use super::error::ParserError;

use {
    force::Force, player::Player, random_item_table::RandomItemTable,
    random_unit_table::RandomUnitTable, tech_availability_change::TechAvailabilityChange,
    upgrade_availability_change::UpgradeAvailabilityChange,
};

pub struct War3MapW3i {
    pub version: u32,
    pub saves: u32,
    pub editor_version: u32,
    pub build_version: Option<[u32; 4]>, // version > 27

    pub name: String,
    pub author: String,
    pub description: String,
    pub recommended_players: String,

    pub camera_bounds: [f32; 8],
    pub camera_bounds_complements: [u32; 4],
    pub playable_size: [u32; 2],

    pub flags: u32,
    pub tileset: u8,

    pub campaign_background: u32,

    pub loading_screen_model: Option<String>, // version > 24
    pub loading_screen_text: String,
    pub loading_screen_title: String,
    pub loading_screen_subtitle: String,
    pub loading_screen: u32,

    pub prologue_screen_model: Option<String>, // version > 24
    pub prologue_screen_text: String,
    pub prologue_screen_title: String,
    pub prologue_screen_subtitle: String,

    // version > 24
    pub use_terrain_fog: Option<u32>,
    pub fog_height: Option<[f32; 2]>,
    pub fog_density: Option<u32>,
    pub fog_color: Option<[u8; 4]>,
    pub global_weather: Option<u32>,
    pub sound_environment: Option<String>,
    pub light_environment_tileset: Option<u8>,
    pub water_vertex_color: Option<[u8; 4]>,

    pub script_mode: Option<u32>, // version > 27

    // version > 30
    pub graphics_mode: Option<u32>,
    pub unknown1: Option<u32>,

    pub players: Vec<Player>,
    pub forces: Vec<Force>,
    pub upgrade_availability_changes: Vec<UpgradeAvailabilityChange>,
    pub tech_availability_changes: Vec<TechAvailabilityChange>,
    pub random_unit_tables: Vec<RandomUnitTable>,
    pub random_item_tables: Vec<RandomItemTable>,
}

impl BinaryReadable for War3MapW3i {
    fn load(stream: &mut BinaryReader, _version: u32) -> Result<Self, ParserError> {
        let version: u32 = AutoReadable::read(stream)?;
        Ok(Self {
            version,
            saves: AutoReadable::read(stream)?,
            editor_version: AutoReadable::read(stream)?,
            build_version: if version > 27 {
                Some(AutoReadable::read(stream)?)
            } else {
                None
            },
            name: AutoReadable::read(stream)?,
            author: AutoReadable::read(stream)?,
            description: AutoReadable::read(stream)?,
            recommended_players: AutoReadable::read(stream)?,
            camera_bounds: AutoReadable::read(stream)?,
            camera_bounds_complements: AutoReadable::read(stream)?,
            playable_size: AutoReadable::read(stream)?,
            flags: AutoReadable::read(stream)?,
            tileset: AutoReadable::read(stream)?,
            campaign_background: AutoReadable::read(stream)?,
            loading_screen_model: if version > 24 {
                Some(AutoReadable::read(stream)?)
            } else {
                None
            },
            loading_screen_text: AutoReadable::read(stream)?,
            loading_screen_title: AutoReadable::read(stream)?,
            loading_screen_subtitle: AutoReadable::read(stream)?,
            loading_screen: AutoReadable::read(stream)?,
            prologue_screen_model: if version > 24 {
                Some(AutoReadable::read(stream)?)
            } else {
                None
            },
            prologue_screen_text: AutoReadable::read(stream)?,
            prologue_screen_title: AutoReadable::read(stream)?,
            prologue_screen_subtitle: AutoReadable::read(stream)?,
            use_terrain_fog: if version > 24 {
                Some(AutoReadable::read(stream)?)
            } else {
                None
            },
            fog_height: if version > 24 {
                Some(AutoReadable::read(stream)?)
            } else {
                None
            },
            fog_density: if version > 24 {
                Some(AutoReadable::read(stream)?)
            } else {
                None
            },
            fog_color: if version > 24 {
                Some(AutoReadable::read(stream)?)
            } else {
                None
            },
            global_weather: if version > 24 {
                Some(AutoReadable::read(stream)?)
            } else {
                None
            },
            sound_environment: if version > 24 {
                Some(AutoReadable::read(stream)?)
            } else {
                None
            },
            light_environment_tileset: if version > 24 {
                Some(AutoReadable::read(stream)?)
            } else {
                None
            },
            water_vertex_color: if version > 24 {
                Some(AutoReadable::read(stream)?)
            } else {
                None
            },
            script_mode: if version > 27 {
                Some(AutoReadable::read(stream)?)
            } else {
                None
            },
            graphics_mode: if version > 30 {
                Some(AutoReadable::read(stream)?)
            } else {
                None
            },
            unknown1: if version > 30 {
                Some(AutoReadable::read(stream)?)
            } else {
                None
            },
            players: {
                let count: u32 = AutoReadable::read(stream)?;
                let mut players: Vec<Player> = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    players.push(Player::load(stream, version)?);
                }
                players
            },
            forces: {
                let count: u32 = AutoReadable::read(stream)?;
                let mut forces: Vec<Force> = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    forces.push(Force::load(stream, version)?);
                }
                forces
            },
            upgrade_availability_changes: {
                let count: u32 = AutoReadable::read(stream)?;
                let mut upgrade_availability_changes: Vec<UpgradeAvailabilityChange> =
                    Vec::with_capacity(count as usize);
                for _ in 0..count {
                    upgrade_availability_changes
                        .push(UpgradeAvailabilityChange::load(stream, version)?);
                }
                upgrade_availability_changes
            },
            tech_availability_changes: {
                let count: u32 = AutoReadable::read(stream)?;
                let mut tech_availability_changes: Vec<TechAvailabilityChange> =
                    Vec::with_capacity(count as usize);
                for _ in 0..count {
                    tech_availability_changes.push(TechAvailabilityChange::load(stream, version)?);
                }
                tech_availability_changes
            },
            random_unit_tables: {
                let count: u32 = AutoReadable::read(stream)?;
                let mut random_unit_tables: Vec<RandomUnitTable> =
                    Vec::with_capacity(count as usize);
                for _ in 0..count {
                    random_unit_tables.push(RandomUnitTable::load(stream, version)?);
                }
                random_unit_tables
            },
            random_item_tables: {
                let count: u32 = AutoReadable::read(stream)?;
                let mut random_item_tables: Vec<RandomItemTable> =
                    Vec::with_capacity(count as usize);
                for _ in 0..count {
                    random_item_tables.push(RandomItemTable::load(stream, version)?);
                }
                random_item_tables
            },
        })
    }
}

impl War3MapW3i {
    pub fn get_build_version(&self) -> u32 {
        match self.build_version {
            Some(version) => version[0] * 100 + version[1],
            None => 0,
        }
    }
}
