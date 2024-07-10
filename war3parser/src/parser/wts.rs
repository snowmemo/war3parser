use std::collections::HashMap;

use regex::Regex;

use crate::extractor::W3Raw;

use super::globals::STRINGS_RE;

type Trigstr = String;

#[derive(Debug)]
pub struct WtsFile {
    pub trigger_strings: HashMap<i32, Trigstr>,
}

impl TryFrom<W3Raw> for WtsFile {
    type Error = &'static str;

    /// Try to convert W3Raw to WtsFile.
    /// Parse raw data to utf8 string.
    ///
    /// build internal hashmap of parsed id and corresponding content.
    fn try_from(w3raw: W3Raw) -> Result<Self, Self::Error> {
        match String::from_utf8(w3raw.data) {
            Ok(buffer) => {
                let re = Regex::new(STRINGS_RE).unwrap();
                let mut trigger_strings = HashMap::new();
                for caps in re.captures_iter(buffer.as_str()) {
                    let id = caps.get(1).unwrap().as_str().to_string();
                    let content = String::from(caps.get(2).unwrap().as_str());
                    if let Ok(id) = id.parse::<i32>() {
                        trigger_strings.insert(id, content);
                    }
                }
                Ok(WtsFile { trigger_strings })
            }
            Err(_) => Err("Failed to convert W3Raw to String"),
        }
    }
}

impl WtsFile {
    pub fn get_ts(&self, id: i32) -> Option<&Trigstr> {
        self.trigger_strings.get(&id)
    }
}
