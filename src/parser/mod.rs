/// globals constants and functions.
pub mod globals;

/// W3Parser trait and some helper functions.
pub mod w3parser;

/// W3I file structure.
///
/// Code inspired by the following sources:
/// 1. <https://github.com/Barogthor/WarEditor/blob/master/wce_map/src/w3i_file.rs>
/// 2. <https://github.com/WaterKnight/Warcraft3-Formats-KaitaiStruct>
pub mod w3i;

/// WTS file structure.
///
/// Code inspired by the following sources:
/// 1. <https://github.com/Barogthor/WarEditor/blob/master/wce_map/src/trigger_string_file.rs>
pub mod wts;

/// W3iParser implementation.
pub mod w3i_parser;
