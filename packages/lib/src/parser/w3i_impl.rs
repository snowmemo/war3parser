use crate::parser::w3i::*;
use crate::parser::w3parser::*;
use nom::{
    combinator::{cond, map},
    multi::count,
    number::complete::{le_f32, le_i32, le_u32, le_u8},
    sequence::tuple,
    IResult,
};

use super::w3str::parse_4char_string;
use super::w3str::parse_cstring;

impl W3BytesParser for GameVersion {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, version) = le_i32(input)?;
        let version = version as u8;

        match version {
            8 | 10 | 11 | 15 | 18 => Ok((
                input,
                GameVersion {
                    version: GameVersionType::RoC,
                    version_number: version,
                },
            )),
            23..=27 => Ok((
                input,
                GameVersion {
                    version: GameVersionType::TFT,
                    version_number: version,
                },
            )),
            28 | 31 => Ok((
                input,
                GameVersion {
                    version: GameVersionType::Reforged,
                    version_number: version,
                },
            )),
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}

impl W3BytesParser for GameVersionCode {
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

impl W3BytesParser for FogStyle {
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

impl W3BytesParser for RandomTablePositionType {
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

impl W3BytesParser for PlayerData {
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

impl W3BytesParser for ForceData {
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

impl W3BytesParser for UpgradeAvailability {
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

impl W3BytesParser for TechAvailability {
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

impl W3BytesParser for RandomUnitTable {
    /// Parses a RandomUnitTable from a byte slice.
    /// Ref: <https://867380699.github.io/blog/2019/05/09/W3X_Files_Format#war3mapw3i:~:text=Random%20unit%20table%20format>
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

impl W3BytesParser for RandomItemSet {
    /// Parses a RandomItemSet from a byte slice.
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, count_items) = le_u32(input)?;
        let (input, items) = count(
            map(tuple((le_u32, parse_cstring)), |(chance, id)| (chance, id)),
            count_items as usize,
        )(input)?;

        Ok((
            input,
            RandomItemSet {
                items: items
                    .into_iter()
                    .map(|(chance, id)| RandomItemSetValue {
                        chance,
                        item_id: id,
                    })
                    .collect(),
            },
        ))
    }
}

impl W3BytesParser for RandomItemTable {
    /// Parses a RandomItemTable from a byte slice.
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, (id, name)) = tuple((le_i32, parse_cstring))(input)?;
        let (input, count_sets) = le_u32(input)?;
        let (input, sets) = count(RandomItemSet::parse, count_sets as usize)(input)?;

        Ok((input, RandomItemTable { id, name, sets }))
    }
}

impl W3BytesParser for Tileset {
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

impl W3BytesParser for W3iFile {
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
            cond(v.version_number > 16, le_i32),
            cond(v.version_number > 16, le_i32),
            cond(v.version_number > 32, GameVersionCode::parse),
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
            cond(
                v.version_number != 18 && v.version_number != 19,
                parse_cstring,
            ),
            parse_cstring,
            parse_cstring,
            parse_cstring,
            cond(v.version_number >= 17, le_i32),
            cond(
                v.version_number != 18 && v.version_number != 19,
                parse_cstring,
            ),
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
            cond(v.version_number >= 19, FogStyle::parse),
            cond(v.version_number >= 21, le_i32),
            cond(v.version_number >= 22, parse_cstring),
            cond(v.version_number >= 23, le_u8),
            cond(v.version_number >= 25, le_u8),
            cond(v.version_number >= 25, le_u8),
            cond(v.version_number >= 25, le_u8),
            cond(v.version_number >= 25, le_u8),
        ))(input)?;

        let (input, (script_language, supported_graphics_modes, game_data_version)) =
            tuple((
                cond(v.version_number >= 28, le_u32),
                cond(v.version_number >= 29, le_i32),
                cond(v.version_number >= 30, le_u32),
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
        let (input, count_random_item_tables) = cond(v.version_number >= 24, le_u32)(input)?;
        let (input, random_item_tables) = cond(
            count_random_item_tables.is_some(),
            count(
                RandomItemTable::parse,
                count_random_item_tables.unwrap() as usize,
            ),
        )(input)?;

        let (input, script_language2) =
            cond(v.version_number == 26 || v.version_number == 27, le_u32)(input)?;

        let flags = MapFlags::new(flags);
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
