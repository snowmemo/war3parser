/*
 *  This file contains the W3I file structure.
 *  The W3I file is a file that contains information about the map.
 *  It is used by the Warcraft III game to load the map.
 *
 *  Code based on https://github.com/Barogthor/WarEditor/blob/master/wce_map/src/w3i_file.rs
 */

use bitfield::bitfield;
use derivative::Derivative;
use enum_display::EnumDisplay;
use nom::{
    bytes::complete::{take, take_until},
    combinator::{cond, map},
    multi::count,
    number::complete::{le_f32, le_i32, le_u32, le_u8},
    sequence::tuple,
    IResult,
};

use crate::extractor::W3Raw;

pub trait W3iParser {
    fn parse(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized;
}

fn parse_4char_string(input: &[u8]) -> IResult<&[u8], String> {
    let (input, bytes) = take(4usize)(input)?;
    let string = String::from_utf8(bytes.to_vec()).unwrap().to_string();
    Ok((input, string))
}

fn parse_cstring(input: &[u8]) -> IResult<&[u8], String> {
    let terminator = "\0";
    let (input, bytes) = take_until(terminator)(input)?;
    let (input, _) = take(1usize)(input)?;
    let string = String::from_utf8(bytes.to_vec()).unwrap().to_string();
    Ok((input, string))
}

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

impl W3iParser for GameVersion {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, version) = le_i32(input)?;
        let version = version as u8;

        match version {
            8 | 10 | 11 | 15 | 18 => Ok((input, GameVersion::RoC(version))),
            23..=27 => Ok((input, GameVersion::TFT(version))),
            28 | 31 => Ok((input, GameVersion::Reforged(version))),
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Fail,
            ))),
        }
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

impl W3iParser for GameVersionCode {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, (major, minor, revision, build)) =
            tuple((le_u32, le_u32, le_u32, le_u32))(input)?;

        Ok((
            input,
            GameVersionCode {
                major,
                minor,
                revision,
                build,
            },
        ))
    }
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

impl W3iParser for FogStyle {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (
            input,
            (
                style,
                z_height_start,
                z_height_end,
                density,
                red_tint,
                green_tint,
                blue_tint,
                alpha_value,
            ),
        ) = tuple((le_i32, le_f32, le_f32, le_f32, le_u8, le_u8, le_u8, le_u8))(input)?;

        Ok((
            input,
            FogStyle {
                style,
                z_height_start,
                z_height_end,
                density,
                red_tint,
                green_tint,
                blue_tint,
                alpha_value,
            },
        ))
    }
}

#[derive(Debug, PartialEq, Clone, PartialOrd, EnumDisplay)]
#[enum_display(case = "Pascal")]
pub enum RandomTablePositionType {
    Unit,
    Building,
    Item,
}

impl W3iParser for RandomTablePositionType {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, position_type) = le_i32(input)?;

        match position_type {
            0 => Ok((input, RandomTablePositionType::Unit)),
            1 => Ok((input, RandomTablePositionType::Building)),
            2 => Ok((input, RandomTablePositionType::Item)),
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RandomUnitSet {
    chance: u32,
    ids: Vec<String>,
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

impl W3iParser for PlayerData {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (
            input,
            (
                player_id,
                player_type,
                player_race,
                fixed_position,
                player_name,
                starting_pos_x,
                starting_pos_y,
                ally_low_priorities,
                ally_high_priorities,
            ),
        ) = tuple((
            le_i32,
            le_i32,
            le_i32,
            le_i32,
            parse_cstring,
            le_f32,
            le_f32,
            le_i32,
            le_i32,
        ))(input)?;

        Ok((
            input,
            PlayerData {
                player_id,
                player_type,
                player_race,
                fixed_position,
                player_name,
                starting_pos_x,
                starting_pos_y,
                ally_low_priorities,
                ally_high_priorities,
            },
        ))
    }
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

impl W3iParser for ForceData {
    /// Parses a ForceData from a byte slice.
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, flags) = le_i32(input)?;
        let allied = flags & 0x0001 != 0;
        let shared_victory = flags & 0x0002 != 0;
        let shared_vision = flags & 0x0004 != 0;
        let shared_unit_control = flags & 0x0010 != 0;
        let shared_advanced_unit_control = flags & 0x0020 != 0;
        let (input, (player_mask, name)) = tuple((le_i32, parse_cstring))(input)?;

        Ok((
            input,
            ForceData {
                flags,
                allied,
                shared_victory,
                shared_vision,
                shared_unit_control,
                shared_advanced_unit_control,
                player_mask,
                name,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct UpgradeAvailability {
    pub player_availability: i32,
    pub upgrade_id: String,
    pub upgrade_level: i32,
    pub availability: i32,
}

impl W3iParser for UpgradeAvailability {
    /// Parses an UpgradeAvailability from a byte slice.
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, (player_availability, upgrade_id, upgrade_level, availability)) =
            tuple((le_i32, parse_4char_string, le_i32, le_i32))(input)?;

        Ok((
            input,
            UpgradeAvailability {
                player_availability,
                upgrade_id,
                upgrade_level,
                availability,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct TechAvailability {
    pub player_availability: u32,
    pub tech_id: String,
}

impl W3iParser for TechAvailability {
    /// Parses a TechAvailability from a byte slice.
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, (player_availability, tech_id)) = tuple((le_u32, parse_4char_string))(input)?;

        Ok((
            input,
            TechAvailability {
                player_availability,
                tech_id,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct RandomUnitTable {
    pub id: i32,
    pub name: String,
    pub position_types: Vec<RandomTablePositionType>,
    pub sets: Vec<RandomUnitSet>,
}

impl W3iParser for RandomUnitTable {
    /// Parses a RandomUnitTable from a byte slice.
    /// Ref: https://867380699.github.io/blog/2019/05/09/W3X_Files_Format#war3mapw3i:~:text=Random%20unit%20table%20format
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, (id, name)) = tuple((le_i32, parse_cstring))(input)?;
        let (input, count_pos) = le_u32(input)?;
        let (input, position_types) =
            count(RandomTablePositionType::parse, count_pos as usize)(input)?;
        let (input, count_sets) = le_u32(input)?;
        let (input, sets) = count(
            map(
                tuple((le_u32, count(parse_cstring, count_pos as usize))),
                |(chance, ids)| RandomUnitSet { chance, ids },
            ),
            count_sets as usize,
        )(input)?;

        Ok((
            input,
            RandomUnitTable {
                id,
                name,
                position_types,
                sets,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct RandomItemSet {
    items: Vec<(u32, String)>,
}

impl W3iParser for RandomItemSet {
    /// Parses a RandomItemSet from a byte slice.
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, count_items) = le_u32(input)?;
        let (input, items) = count(
            map(tuple((le_u32, parse_cstring)), |(chance, id)| (chance, id)),
            count_items as usize,
        )(input)?;

        Ok((input, RandomItemSet { items }))
    }
}

#[derive(Debug, PartialEq)]
pub struct RandomItemTable {
    pub id: i32,
    pub name: String,
    pub sets: Vec<RandomItemSet>,
}

impl W3iParser for RandomItemTable {
    /// Parses a RandomItemTable from a byte slice.
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, (id, name)) = tuple((le_i32, parse_cstring))(input)?;
        let (input, count_sets) = le_u32(input)?;
        let (input, sets) = count(RandomItemSet::parse, count_sets as usize)(input)?;

        Ok((input, RandomItemTable { id, name, sets }))
    }
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

impl W3iParser for Tileset {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, tileset) = le_u8(input)?;
        let tileset = tileset as char;
        let tileset = match tileset {
            'A' => Tileset::Ashenvale,
            'B' => Tileset::Barrens,
            'C' => Tileset::Felwood,
            'D' => Tileset::Dungeon,
            'F' => Tileset::LordaeronFall,
            'G' => Tileset::Underground,
            'I' => Tileset::Icecrown,
            'J' => Tileset::DalaranRuins,
            'K' => Tileset::BlackCitadel,
            'L' => Tileset::LordaeronSummer,
            'N' => Tileset::Northrend,
            'O' => Tileset::Outland,
            'Q' => Tileset::VillageFall,
            'V' => Tileset::Village,
            'W' => Tileset::LordaeronWinter,
            'X' => Tileset::Dalaran,
            'Y' => Tileset::Cityscape,
            'Z' => Tileset::SunkenRuins,
            _ => Tileset::Known,
        };
        Ok((input, tileset))
    }
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

impl W3iParser for W3iFile {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, v) = GameVersion::parse(input)?;
        let (
            input,
            (
                count_saves,
                editor_version,
                game_version,
                map_name,
                map_author,
                map_description,
                recommended_players,
                camera_bounds,
                camera_bounds_complements,
                map_playable_width,
                map_playable_height,
                flags,
            ),
        ) = tuple((
            cond(v > 16, le_i32),
            cond(v > 16, le_i32),
            cond(v > 32, GameVersionCode::parse),
            parse_cstring,
            parse_cstring,
            parse_cstring,
            parse_cstring,
            count(le_f32, 8),
            count(le_i32, 4),
            le_i32,
            le_i32,
            le_u32,
        ))(input)?;

        let (input, tileset) = Tileset::parse(input)?;

        let (
            input,
            (
                loading_screen_index,
                custom_loading_screen_model_path,
                loading_screen_text,
                loading_screen_title,
                loading_screen_subtitle,
                user_game_dataset,
                prologue_screen_path,
                prologue_screen_text,
                prologue_screen_title,
                prologue_screen_subtitle,
            ),
        ) = tuple((
            le_i32,
            cond(v != 18 && v != 19, parse_cstring),
            parse_cstring,
            parse_cstring,
            parse_cstring,
            cond(v >= 17, le_i32),
            cond(v != 18 && v != 19, parse_cstring),
            parse_cstring,
            parse_cstring,
            parse_cstring,
        ))(input)?;

        let (
            input,
            (
                fog_style,
                global_weather,
                custom_sound_environment,
                custom_light_environment_id,
                custom_water_red_tint,
                custom_water_green_tint,
                custom_water_blue_tint,
                custom_water_alpha_tint,
            ),
        ) = tuple((
            cond(v >= 19, FogStyle::parse),
            cond(v >= 21, le_i32),
            cond(v >= 22, parse_cstring),
            cond(v >= 23, le_u8),
            cond(v >= 25, le_u8),
            cond(v >= 25, le_u8),
            cond(v >= 25, le_u8),
            cond(v >= 25, le_u8),
        ))(input)?;

        let (input, (script_language, supported_graphics_modes, game_data_version)) =
            tuple((
                cond(v >= 28, le_u32),
                cond(v >= 29, le_i32),
                cond(v >= 30, le_u32),
            ))(input)?;

        let (input, count_players) = le_i32(input)?;
        let (input, players) = count(PlayerData::parse, count_players as usize)(input)?;
        let (input, count_forces) = le_i32(input)?;
        let (input, forces) = count(ForceData::parse, count_forces as usize)(input)?;
        let (input, count_upgrades) = le_i32(input)?;
        let (input, upgrades) = count(UpgradeAvailability::parse, count_upgrades as usize)(input)?;
        let (input, count_techs) = le_i32(input)?;
        let (input, techs) = count(TechAvailability::parse, count_techs as usize)(input)?;
        let (input, count_random_unit_tables) = le_i32(input)?;
        let (input, random_unit_tables) =
            count(RandomUnitTable::parse, count_random_unit_tables as usize)(input)?;
        let (input, count_random_item_tables) = cond(v >= 24, le_u32)(input)?;
        let (input, random_item_tables) = cond(
            count_random_item_tables.is_some(),
            count(
                RandomItemTable::parse,
                count_random_item_tables.unwrap() as usize,
            ),
        )(input)?;

        let (input, script_language2) = cond(v == 26 || v == 27, le_u32)(input)?;

        let flags = MapFlags(flags);
        let map_width =
            camera_bounds_complements[0] + camera_bounds_complements[1] + map_playable_width;
        let map_height =
            camera_bounds_complements[2] + camera_bounds_complements[3] + map_playable_height;
        let map_size = MapSize::from_area(map_playable_height * map_playable_width);
        Ok((
            input,
            W3iFile {
                version: v,
                count_saves,
                editor_version,
                game_version,
                map_name,
                map_author,
                map_description,
                recommended_players,
                camera_bounds,
                camera_bounds_complements,
                map_playable_width,
                map_playable_height,
                map_width,
                map_height,
                map_size,
                flags,
                tileset,
                loading_screen_index,
                custom_loading_screen_model_path,
                loading_screen_text,
                loading_screen_title,
                loading_screen_subtitle,
                user_game_dataset,
                prologue_screen_path,
                prologue_screen_text,
                prologue_screen_title,
                prologue_screen_subtitle,
                fog_style,
                global_weather,
                custom_sound_environment,
                custom_light_environment_id: custom_light_environment_id.map(|c| c as char),
                custom_water_red_tint,
                custom_water_green_tint,
                custom_water_blue_tint,
                custom_water_alpha_tint,
                script_language,
                supported_graphics_modes,
                game_data_version,
                players,
                forces,
                upgrades,
                techs,
                random_unit_tables,
                random_item_tables,
                script_language2,
            },
        ))
    }
}

impl TryFrom<W3Raw> for W3iFile {
    type Error = &'static str;

    fn try_from(w3raw: W3Raw) -> Result<Self, Self::Error> {
        match W3iFile::parse(&w3raw.data) {
            Ok((_, w3i)) => Ok(w3i),
            Err(_) => Err("Failed to parse W3I file"),
        }
    }
}
