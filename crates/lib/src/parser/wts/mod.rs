use std::collections::HashMap;

use regex::Regex;

use super::error::ParserError;

/// TRIGSTR_007 / TRIGSTR_007ab / TRIGSTR_7 TRIGSTR_-007
pub const TRAGGER_STR_RE: &str = r"TRIGSTR_(-?\d+)(?:\w+)?";

/// STRING_007
pub const STRINGS_RE: &str = r"STRING\s+([0-9]+)\s+\{\r\n+([^\}]*)\r\n\}";

/// String table
pub struct War3MapWts {
    pub string_map: HashMap<i32, String>,
}

/// Load a string table from a buffer
impl War3MapWts {
    pub fn load(buffer: &str) -> Result<Self, ParserError> {
        let re = Regex::new(STRINGS_RE)?;
        let mut string_map = HashMap::new();
        for caps in re.captures_iter(buffer) {
            let id = caps
                .get(1)
                .ok_or(ParserError::FailedToFindStrings)?
                .as_str()
                .to_string();
            let content = String::from(
                caps.get(2)
                    .ok_or(ParserError::FailedToFindStrings)?
                    .as_str(),
            );
            if let Ok(id) = id.parse::<i32>() {
                string_map.insert(id, content);
            }
        }
        Ok(Self { string_map })
    }
}
