use derivative::Derivative;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::extractor::W3Raw;
use crate::parser::w3parser::W3BytesParser;

use super::globals::TRAGGER_STR_RE;
use super::w3parser::get_bit_from_u32;
use super::wts::WtsFile;

// https://github.com/stijnherfst/HiveWE/blob/c3ba5542d0e28e8c537c0aaf25269d9f6bd75d5d/src/base/map_info.ixx#L176
// TODO: Implement the rest of the W3I file 

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum GameVersionType {
    RoC,
    TFT,
    Reforged,
    Known,
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct GameVersion {
    pub version: GameVersionType,
    pub version_number: u8,
}

impl Default for GameVersion {
    fn default() -> Self {
        GameVersion {
            version: GameVersionType::TFT,
            version_number: 25,
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, Clone)]
#[wasm_bindgen]
pub struct MapFlags {
    pub flags: u32,
    pub hide_minimap_on_preview_screens: bool,
    pub change_ally_priorities: bool,
    pub melee: bool,
    pub non_default_tileset_map_size_large_never_been_reduced_to_medium: bool,
    pub unexplored_areas_partially_visible: bool,
    pub fixed_player_parameters_for_custom_teams: bool,
    pub use_custom_teams: bool,
    pub use_custom_techs: bool,

    pub use_custom_abilities: bool,
    pub use_custom_upgrades: bool,
    pub map_properties_menu_opened_at_least_once: bool,
    pub show_water_waves_on_cliff_shores: bool,
    pub show_water_waves_on_rolling_shores: bool,
    pub use_terrain_fog: bool,
    pub tft_required: bool,
    pub use_item_classification_system: bool,

    pub use_accurate_probabilities_for_calculation: bool,
    pub use_custom_ability_skin: bool,
    pub flag19: bool,
    pub flag18: bool,
    pub flag17: bool,
    pub custom_water_tint_color: bool,
}

impl MapFlags {
    pub fn new(flags: u32) -> Self {
        MapFlags {
            flags,
            hide_minimap_on_preview_screens: get_bit_from_u32(flags, 0),
            change_ally_priorities: get_bit_from_u32(flags, 1),
            melee: get_bit_from_u32(flags, 2),
            non_default_tileset_map_size_large_never_been_reduced_to_medium: get_bit_from_u32(
                flags, 3,
            ),
            unexplored_areas_partially_visible: get_bit_from_u32(flags, 4),
            fixed_player_parameters_for_custom_teams: get_bit_from_u32(flags, 5),
            use_custom_teams: get_bit_from_u32(flags, 6),
            use_custom_techs: get_bit_from_u32(flags, 7),

            use_custom_abilities: get_bit_from_u32(flags, 8),
            use_custom_upgrades: get_bit_from_u32(flags, 9),
            map_properties_menu_opened_at_least_once: get_bit_from_u32(flags, 10),
            show_water_waves_on_cliff_shores: get_bit_from_u32(flags, 11),
            show_water_waves_on_rolling_shores: get_bit_from_u32(flags, 12),
            use_terrain_fog: get_bit_from_u32(flags, 13),
            tft_required: get_bit_from_u32(flags, 14),
            use_item_classification_system: get_bit_from_u32(flags, 15),

            use_accurate_probabilities_for_calculation: get_bit_from_u32(flags, 16),
            use_custom_ability_skin: get_bit_from_u32(flags, 17),
            flag19: get_bit_from_u32(flags, 18),
            flag18: get_bit_from_u32(flags, 19),
            flag17: get_bit_from_u32(flags, 20),
            custom_water_tint_color: get_bit_from_u32(flags, 21),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[wasm_bindgen]
pub struct GameVersionCode {
    pub major: u32,
    pub minor: u32,
    pub revision: u32,
    pub build: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[wasm_bindgen]
pub struct FogStyle {
    pub style: i32, // v >= 19
    pub z_height_start: f32,
    pub z_height_end: f32,
    pub density: f32,
    pub red_tint: u8,
    pub green_tint: u8,
    pub blue_tint: u8,
    pub alpha_value: u8,
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum RandomTablePositionType {
    Unit,
    Building,
    Item,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct RandomUnitSet {
    pub chance: u32,
    pub ids: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct PlayerData {
    pub player_id: i32,
    pub player_type: i32,
    pub player_race: i32,
    pub fixed_position: i32,
    pub player_name: String,
    pub starting_pos_x: f32,
    pub starting_pos_y: f32,
    pub ally_low_priorities: i32,
    pub ally_high_priorities: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct ForceData {
    pub flags: i32,
    pub allied: bool,
    pub shared_victory: bool,
    pub shared_vision: bool,
    pub shared_unit_control: bool,
    pub shared_advanced_unit_control: bool,
    pub player_mask: i32,
    pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct UpgradeAvailability {
    pub player_availability: i32,
    pub upgrade_id: String,
    pub upgrade_level: i32,
    pub availability: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct TechAvailability {
    pub player_availability: u32,
    pub tech_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct RandomUnitTable {
    pub id: i32,
    pub name: String,
    pub position_types: Vec<RandomTablePositionType>,
    pub sets: Vec<RandomUnitSet>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct RandomItemSetValue {
    pub chance: u32,
    pub item_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct RandomItemSet {
    pub items: Vec<RandomItemSetValue>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct RandomItemTable {
    pub id: i32,
    pub name: String,
    pub sets: Vec<RandomItemSet>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, Clone)]
#[wasm_bindgen]
pub enum Tileset {
    #[default]
    Ashenvale,
    Barrens,
    Felwood,
    Dungeon,
    LordaeronFall,
    Underground,
    Icecrown,
    DalaranRuins,
    BlackCitadel,
    LordaeronSummer,
    Northrend,
    Outland,
    VillageFall,
    Village,
    LordaeronWinter,
    Dalaran,
    Cityscape,
    SunkenRuins,
    Known,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, Clone)]
#[wasm_bindgen]
pub enum MapSize {
    #[default]
    Tiny,
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
    Huge,
    Epic,
    Legendary,
    Unknown,
}

impl MapSize {
    pub fn from_area(area: i32) -> Self {
        match area {
            45..=6600 => MapSize::Tiny,
            6601..=12800 => MapSize::ExtraSmall,
            12801..=21000 => MapSize::Small,
            21001..=31000 => MapSize::Medium,
            31001..=43500 => MapSize::Large,
            43501..=74000 => MapSize::ExtraLarge,
            74001..=135000 => MapSize::Huge,
            135001..=215000 => MapSize::Epic,
            215001..=230400 => MapSize::Legendary,
            _ => MapSize::Unknown,
        }
    }
}

#[derive(Derivative, Serialize, Deserialize, Clone)]
#[derivative(Debug, Default, PartialEq)]
#[wasm_bindgen(getter_with_clone)]
pub struct W3iFile {
    pub version: GameVersion,
    #[derivative(PartialEq = "ignore")]
    pub count_saves: Option<i32>, // v >= 16
    #[derivative(PartialEq = "ignore")]
    pub editor_version: Option<i32>, // v >= 16
    pub game_version: Option<GameVersionCode>, // v >= 28
    pub map_name: String,
    pub map_author: String,
    pub map_description: String,
    pub recommended_players: String,
    pub camera_bounds: Vec<f32>,
    pub camera_bounds_complements: Vec<i32>,
    pub map_playable_width: i32,
    pub map_playable_height: i32,
    pub map_width: i32,
    pub map_height: i32,
    pub map_size: MapSize,
    pub flags: MapFlags,

    pub tileset: Tileset,
    pub loading_screen_index: i32,
    pub custom_loading_screen_model_path: Option<String>, // v != 18 && v != 19
    pub loading_screen_text: String,
    pub loading_screen_title: String,
    pub loading_screen_subtitle: String,
    pub user_game_dataset: Option<i32>,       // v >= 17
    pub prologue_screen_path: Option<String>, // v != 18 && v != 19
    pub prologue_screen_text: String,
    pub prologue_screen_title: String,
    pub prologue_screen_subtitle: String,

    pub fog_style: Option<FogStyle>,               // v >= 19
    pub global_weather: Option<i32>,               // v >= 21
    pub custom_sound_environment: Option<String>,  // v >= 22
    pub custom_light_environment_id: Option<char>, // v >= 23
    pub custom_water_red_tint: Option<u8>,         // v >= 25
    pub custom_water_green_tint: Option<u8>,       // v >= 25
    pub custom_water_blue_tint: Option<u8>,        // v >= 25
    pub custom_water_alpha_tint: Option<u8>,       // v >= 25

    pub script_language: Option<u32>,          // v >= 28
    pub supported_graphics_modes: Option<i32>, // v >= 29
    pub game_data_version: Option<u32>,        // v >= 30

    pub players: Vec<PlayerData>,
    pub forces: Vec<ForceData>,
    pub upgrades: Vec<UpgradeAvailability>,
    pub techs: Vec<TechAvailability>,
    pub random_unit_tables: Vec<RandomUnitTable>,
    pub random_item_tables: Option<Vec<RandomItemTable>>, // v >= 24
    pub script_language2: Option<u32>,                    // v == 26 or v == 27
}

impl W3iFile {
    pub fn try_from_w3raw(w3raw: W3Raw) -> Option<W3iFile> {
        match <W3iFile>::parse(&w3raw.data) {
            Ok((_, w3i)) => Some(w3i),
            Err(_) => None,
        }
    }

    pub fn update_with_wts(&self, wts: &WtsFile) -> Self {
        self.update_with_hashmap(&wts.trigger_strings)
    }

    /// Extract trigger strings from the W3I file
    ///
    /// returns a hashmap of original trigger strings and corresponding id
    pub fn trigger_strings(&self) -> HashMap<String, i32> {
        let re = Regex::new(TRAGGER_STR_RE).unwrap();
        let json = serde_json::to_string(&self).unwrap();
        let mut trigger_strings = HashMap::new();
        for caps in re.captures_iter(json.as_str()) {
            let original = caps.get(0).unwrap().as_str().to_string();
            let id = match caps.get(1) {
                Some(id) => id.as_str(),
                None => "0",
            };
            if let Ok(id) = id.parse::<i32>() {
                if id > 0 {
                    trigger_strings.insert(original, id);
                }
            };
        }
        trigger_strings
    }

    pub fn update_with_hashmap(&self, hash: &HashMap<i32, String>) -> Self {
        let mut json = serde_json::to_string(&self).unwrap();
        let trigger_strings = self.trigger_strings();
        let default = "".to_string();
        trigger_strings.iter().for_each(|(original, id)| {
            let replacement = hash.get(id).unwrap_or(&default);
            json = json.replace(original, replacement);
        });
        serde_json::from_str(&json).unwrap()
    }
}
