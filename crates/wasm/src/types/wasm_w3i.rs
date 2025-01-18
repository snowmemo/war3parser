use js_sys::{Float32Array, Uint32Array, Uint8Array};
use war3parser::parser::w3i::{
    force::Force,
    player::Player,
    random_item_table::{RandomItem, RandomItemSet, RandomItemTable},
    random_unit_table::{RandomUnit, RandomUnitTable},
    tech_availability_change::TechAvailabilityChange,
    upgrade_availability_change::UpgradeAvailabilityChange,
    War3MapW3i,
};
use wasm_bindgen::prelude::wasm_bindgen;

use super::utils::{f32_array_to_float32array, u32_array_to_uint32array, u8_array_to_uint8array};

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct WasmPlayer {
    pub id: i32,
    pub player_type: i32,
    pub race: i32,
    pub is_fixed_start_position: i32,
    pub name: String,
    pub start_location: Float32Array,
    pub ally_low_priorities: u32,
    pub ally_high_priorities: u32,
    pub known1: Option<Uint8Array>,
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct WasmForce {
    pub flags: u32,
    pub player_masks: u32,
    pub name: String,
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct WasmRandomItem {
    pub chance: i32,
    pub id: Uint8Array,
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct WasmRandomItemSet {
    pub items: Vec<WasmRandomItem>,
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct WasmRandomItemTable {
    pub id: i32,
    pub name: String,
    pub sets: Vec<WasmRandomItemSet>,
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct WasmRandomUnit {
    pub chance: i32,
    pub ids: Vec<Uint8Array>,
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct WasmRandomUnitTable {
    pub id: i32,
    pub name: String,
    pub position: i32,
    pub column_types: Vec<i32>,
    pub units: Vec<WasmRandomUnit>,
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct WasmTechAvailabilityChange {
    pub player_flags: u32,
    pub id: Uint8Array,
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct WasmUpgradeAvailabilityChange {
    pub player_flags: u32,
    pub id: Uint8Array,
    pub level_affected: i32,
    pub availability: i32,
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct WasmW3i {
    pub version: u32,
    pub saves: u32,
    pub editor_version: u32,
    pub build_version: Option<Uint32Array>, // version > 27

    pub name: String,
    pub author: String,
    pub description: String,
    pub recommended_players: String,

    pub camera_bounds: Float32Array,
    pub camera_bounds_complements: Uint32Array,
    pub playable_size: Uint32Array,

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
    pub fog_height: Option<Float32Array>,
    pub fog_density: Option<u32>,
    pub fog_color: Option<Uint8Array>,
    pub global_weather: Option<u32>,
    pub sound_environment: Option<String>,
    pub light_environment_tileset: Option<u8>,
    pub water_vertex_color: Option<Uint8Array>,

    pub script_mode: Option<u32>, // version > 27

    // version > 30
    pub graphics_mode: Option<u32>,
    pub unknown1: Option<u32>,

    pub players: Vec<WasmPlayer>,
    pub forces: Vec<WasmForce>,
    pub upgrade_availability_changes: Vec<WasmUpgradeAvailabilityChange>,
    pub tech_availability_changes: Vec<WasmTechAvailabilityChange>,
    pub random_unit_tables: Vec<WasmRandomUnitTable>,
    pub random_item_tables: Vec<WasmRandomItemTable>,
}

impl From<Player> for WasmPlayer {
    fn from(player: Player) -> Self {
        Self {
            id: player.id,
            player_type: player.player_type,
            race: player.race,
            is_fixed_start_position: player.is_fixed_start_position,
            name: player.name,
            start_location: f32_array_to_float32array(player.start_location),
            ally_low_priorities: player.ally_low_priorities,
            ally_high_priorities: player.ally_high_priorities,
            known1: player.known1.map(u8_array_to_uint8array),
        }
    }
}

impl From<Force> for WasmForce {
    fn from(force: Force) -> Self {
        Self {
            flags: force.flags,
            player_masks: force.player_masks,
            name: force.name,
        }
    }
}

impl From<RandomItem> for WasmRandomItem {
    fn from(item: RandomItem) -> Self {
        Self {
            chance: item.chance,
            id: u8_array_to_uint8array(item.id),
        }
    }
}

impl From<RandomItemSet> for WasmRandomItemSet {
    fn from(set: RandomItemSet) -> Self {
        Self {
            items: set.items.into_iter().map(WasmRandomItem::from).collect(),
        }
    }
}

impl From<RandomItemTable> for WasmRandomItemTable {
    fn from(table: RandomItemTable) -> Self {
        Self {
            id: table.id,
            name: table.name,
            sets: table
                .sets
                .into_iter()
                .map(WasmRandomItemSet::from)
                .collect(),
        }
    }
}

impl From<RandomUnit> for WasmRandomUnit {
    fn from(unit: RandomUnit) -> Self {
        Self {
            chance: unit.chance,
            ids: unit.ids.into_iter().map(u8_array_to_uint8array).collect(),
        }
    }
}

impl From<RandomUnitTable> for WasmRandomUnitTable {
    fn from(table: RandomUnitTable) -> Self {
        Self {
            id: table.id,
            name: table.name,
            position: table.position,
            column_types: table.column_types,
            units: table.units.into_iter().map(WasmRandomUnit::from).collect(),
        }
    }
}

impl From<TechAvailabilityChange> for WasmTechAvailabilityChange {
    fn from(change: TechAvailabilityChange) -> Self {
        Self {
            player_flags: change.player_flags,
            id: u8_array_to_uint8array(change.id),
        }
    }
}

impl From<UpgradeAvailabilityChange> for WasmUpgradeAvailabilityChange {
    fn from(change: UpgradeAvailabilityChange) -> Self {
        Self {
            player_flags: change.player_flags,
            id: u8_array_to_uint8array(change.id),
            level_affected: change.level_affected,
            availability: change.availability,
        }
    }
}

impl From<War3MapW3i> for WasmW3i {
    fn from(w3i: War3MapW3i) -> Self {
        Self {
            version: w3i.version,
            saves: w3i.saves,
            editor_version: w3i.editor_version,
            build_version: w3i.build_version.map(u32_array_to_uint32array),
            name: w3i.name,
            author: w3i.author,
            description: w3i.description,
            recommended_players: w3i.recommended_players,
            camera_bounds: f32_array_to_float32array(w3i.camera_bounds),
            camera_bounds_complements: u32_array_to_uint32array(w3i.camera_bounds_complements),
            playable_size: u32_array_to_uint32array(w3i.playable_size),
            flags: w3i.flags,
            tileset: w3i.tileset,
            campaign_background: w3i.campaign_background,

            loading_screen_model: w3i.loading_screen_model,
            loading_screen_text: w3i.loading_screen_text,
            loading_screen_title: w3i.loading_screen_title,
            loading_screen_subtitle: w3i.loading_screen_subtitle,
            loading_screen: w3i.loading_screen,

            prologue_screen_model: w3i.prologue_screen_model,
            prologue_screen_text: w3i.prologue_screen_text,
            prologue_screen_title: w3i.prologue_screen_title,
            prologue_screen_subtitle: w3i.prologue_screen_subtitle,

            use_terrain_fog: w3i.use_terrain_fog,
            fog_height: w3i.fog_height.map(f32_array_to_float32array),
            fog_density: w3i.fog_density,
            fog_color: w3i.fog_color.map(u8_array_to_uint8array),
            global_weather: w3i.global_weather,
            sound_environment: w3i.sound_environment,
            light_environment_tileset: w3i.light_environment_tileset,
            water_vertex_color: w3i.water_vertex_color.map(u8_array_to_uint8array),
            script_mode: w3i.script_mode,
            graphics_mode: w3i.graphics_mode,
            unknown1: w3i.unknown1,

            players: w3i.players.into_iter().map(WasmPlayer::from).collect(),
            forces: w3i.forces.into_iter().map(WasmForce::from).collect(),
            upgrade_availability_changes: w3i
                .upgrade_availability_changes
                .into_iter()
                .map(WasmUpgradeAvailabilityChange::from)
                .collect(),
            tech_availability_changes: w3i
                .tech_availability_changes
                .into_iter()
                .map(WasmTechAvailabilityChange::from)
                .collect(),
            random_unit_tables: w3i
                .random_unit_tables
                .into_iter()
                .map(WasmRandomUnitTable::from)
                .collect(),
            random_item_tables: w3i
                .random_item_tables
                .into_iter()
                .map(WasmRandomItemTable::from)
                .collect(),
        }
    }
}
