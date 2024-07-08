use bitfield::bitfield;
use derivative::Derivative;
use enum_display::EnumDisplay;

use crate::extractor::W3Raw;
use crate::parser::w3parser::W3Parser;

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy, EnumDisplay)]
#[enum_display(case = "Pascal")]
pub enum GameVersion {
    RoC(u8),
    TFT(u8),
    Reforged(u8),
    Known(u8),
}

impl AsRef<u8> for GameVersion {
    fn as_ref(&self) -> &u8 {
        match self {
            GameVersion::RoC(version) => version,
            GameVersion::TFT(version) => version,
            GameVersion::Reforged(version) => version,
            GameVersion::Known(version) => version,
        }
    }
}

impl PartialOrd<u8> for GameVersion {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.as_ref().partial_cmp(other)
    }
}

impl PartialEq<u8> for GameVersion {
    fn eq(&self, other: &u8) -> bool {
        self.as_ref() == other
    }
}

impl Default for GameVersion {
    fn default() -> Self {
        GameVersion::TFT(25)
    }
}

bitfield! {
    #[derive(Default,PartialEq)]
    pub struct MapFlags(u32);
    impl Debug;
    pub hide_minimap_on_preview_screens, set_hide_minimap_on_preview_screens: 0;
    pub change_ally_priorities, set_change_ally_priorities: 1;
    pub melee, set_melee: 2;
    pub non_default_tileset_map_size_large_never_been_reduced_to_medium, set_non_default_tileset_map_size_large_never_been_reduced_to_medium: 3;
    pub unexplored_areas_partially_visible, set_unexplored_areas_partially_visible: 4;
    pub fixed_player_parameters_for_custom_teams, set_fixed_player_parameters_for_custom_teams: 5;
    pub use_custom_teams, set_use_custom_teams: 6;
    pub use_custom_techs, set_use_custom_techs: 7;

    pub use_custom_abilities, set_use_custom_abilities: 8;
    pub use_custom_upgrades, set_use_custom_upgrades: 9;
    pub map_properties_menu_opened_at_least_once, set_map_properties_menu_opened_at_least_once: 10;
    pub show_water_waves_on_cliff_shores, set_show_water_waves_on_cliff_shores: 11;
    pub show_water_waves_on_rolling_shores, set_show_water_waves_on_rolling_shores: 12;
    pub use_terrain_fog, set_use_terrain_fog: 13;
    pub tft_required, set_tft_required: 14;
    pub use_item_classification_system, set_use_item_classification_system: 15;

    pub use_accurate_probabilities_for_calculation, set_use_accurate_probabilities_for_calculation: 17;
    pub use_custom_ability_skin, set_use_custom_ability_skin: 18;
    pub flag19, set_flag19: 19;
    pub flag18, set_flag18: 20;
    pub flag17, set_flag17: 21;
    pub custom_water_tint_color, set_custom_water_tint_color: 22;
}

#[derive(Debug, PartialEq)]
pub struct GameVersionCode {
    pub major: u32,
    pub minor: u32,
    pub revision: u32,
    pub build: u32,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq, Clone, PartialOrd, EnumDisplay)]
#[enum_display(case = "Pascal")]
pub enum RandomTablePositionType {
    Unit,
    Building,
    Item,
}

#[derive(Debug, PartialEq)]
pub struct RandomUnitSet {
    pub chance: u32,
    pub ids: Vec<String>,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct UpgradeAvailability {
    pub player_availability: i32,
    pub upgrade_id: String,
    pub upgrade_level: i32,
    pub availability: i32,
}

#[derive(Debug, PartialEq)]
pub struct TechAvailability {
    pub player_availability: u32,
    pub tech_id: String,
}

#[derive(Debug, PartialEq)]
pub struct RandomUnitTable {
    pub id: i32,
    pub name: String,
    pub position_types: Vec<RandomTablePositionType>,
    pub sets: Vec<RandomUnitSet>,
}

#[derive(Debug, PartialEq)]
pub struct RandomItemSet {
    pub items: Vec<(u32, String)>,
}

#[derive(Debug, PartialEq)]
pub struct RandomItemTable {
    pub id: i32,
    pub name: String,
    pub sets: Vec<RandomItemSet>,
}

#[derive(Debug, PartialEq, Default, EnumDisplay)]
#[enum_display(case = "Pascal")]
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

#[derive(Debug, PartialEq, Default, EnumDisplay)]
#[enum_display(case = "Pascal")]
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

#[derive(Derivative)]
#[derivative(Debug, Default, PartialEq)]
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

impl TryFrom<W3Raw> for W3iFile {
    type Error = &'static str;

    fn try_from(w3raw: W3Raw) -> Result<Self, Self::Error> {
        match <W3iFile>::parse(&w3raw.data) {
            Ok((_, w3i)) => Ok(w3i),
            Err(_) => Err("Failed to parse W3I file"),
        }
    }
}
