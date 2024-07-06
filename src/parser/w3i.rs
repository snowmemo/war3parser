/*
 *  This file contains the W3I file structure.
 *  The W3I file is a file that contains information about the map.
 *  It is used by the Warcraft III game to load the map.
 *
 *  Code based on https://github.com/Barogthor/WarEditor/blob/master/wce_map/src/w3i_file.rs
 */

use derivative::Derivative;

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum GameVersion {
    RoC,
    TFT,
    Reforged,
}

impl Default for GameVersion {
    fn default() -> Self {
        GameVersion::TFT
    }
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum RandomTablePositionType {
    Unit,
    Building,
    Item,
}

#[derive(Debug, PartialEq)]
struct RandomUnitSet {
    chance: u32,
    ids: Vec<String>,
}

#[derive(Debug, PartialEq)]
struct PlayerData {
    player_id: i32,
    player_type: i32,
    player_race: i32,
    fixed_position: i32,
    player_name: String,
    starting_pos_x: f32,
    starting_pos_y: f32,
    ally_low_priorities: i32,
    ally_high_priorities: i32,
}

#[derive(Debug, PartialEq)]
struct ForceData {
    flags: i32,
    allied: bool,
    shared_victory: bool,
    shared_vision: bool,
    shared_unit_control: bool,
    shared_advanced_unit_control: bool,
    player_mask: i32,
    name: String,
}

#[derive(Debug, PartialEq)]
struct UpgradeAvailability {
    player_availability: i32,
    upgrade_id: String,
    upgrade_level: i32,
    availability: i32,
}

#[derive(Debug, PartialEq)]
struct TechAvailability {
    player_availability: i32,
    tech_id: String,
}

#[derive(Debug, PartialEq)]
struct RandomUnitTable {
    id: i32,
    name: String,
    position_types: Vec<RandomTablePositionType>,
    sets: Vec<RandomUnitSet>,
}

#[derive(Debug, PartialEq)]
struct RandomItemSet {
    items: Vec<(u32, String)>,
}

#[derive(Debug, PartialEq)]
pub struct RandomItemTable {
    id: i32,
    name: String,
    sets: Vec<RandomItemSet>,
}

#[derive(Derivative)]
#[derivative(Debug, Default, PartialEq)]
pub struct W3iFile {
    version: GameVersion,
    #[derivative(PartialEq = "ignore")]
    count_saves: i32,
    #[derivative(PartialEq = "ignore")]
    editor_version: i32,
    map_name: String,
    map_author: String,
    map_description: String,
    recommended_players: String,
    camera_bounds: Vec<f32>,             // 8
    camera_bounds_complements: Vec<i32>, // 4
    map_playable_width: i32,
    map_playable_height: i32,

    flags: i32,
    hide_minimap_preview: bool,
    modifiy_ally_priorities: bool,
    is_melee: bool,
    unknown: bool,
    mask_partial_vision: bool,
    fixed_custom_player_force: bool,
    use_custom_force: bool,
    use_custom_tree: bool,
    use_custom_abilities: bool,
    use_custom_upgrades: bool,
    unkwown_2: bool,
    show_waves_cliff_shores: bool,
    show_waves_rolling_shores: bool,
    unkwown_3: bool, // TFT
    unkwown_4: bool, // TFT
    unkwown_5: bool, // TFT

    ground_type: char,
    campaign_background: i32,                 // RoC
    custom_loading_screen_model_path: String, // TFT
    loading_screen_index: i32,
    loading_screen_text: String,
    loading_screen_title: String,
    loading_screen_subtitle: String,
    user_game_dataset: i32,       // TFT
    prologue_screen_path: String, // TFT
    prologue_screen_text: String,
    prologue_screen_title: String,
    prologue_screen_subtitle: String,
    // TFT
    fog_style: i32,
    fog_z_height_start: f32,
    fog_z_height_end: f32,
    fog_density: f32,
    fog_red_tint: u8,
    fog_green_tint: u8,
    fog_blue_tint: u8,
    fog_alpha_value: u8,
    global_weather: i32,
    custom_sound_environment: String,
    custom_light_environment_id: char,
    custom_water_red_tint: u8,
    custom_water_green_tint: u8,
    custom_water_blue_tint: u8,
    custom_water_alpha_tint: u8,

    players: Vec<PlayerData>,
    forces: Vec<ForceData>,
    upgrades: Vec<UpgradeAvailability>,
    techs: Vec<TechAvailability>,
    random_unit_tables: Vec<RandomUnitTable>,
    random_item_tables: Vec<RandomItemTable>,
}
