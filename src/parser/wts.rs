use std::collections::HashMap;

use regex::Regex;

use crate::extractor::W3Raw;

const EXTRACT_DATA: &str = r"STRING\s+([0-9]+)\s+\{\r\n+([^\}]*)\r\n\}";
type Trigstr = String;

#[derive(Debug)]
pub struct WtsFile {
    pub trigger_strings: HashMap<String, Trigstr>,
}

impl WtsFile {
    pub fn from_w3raw(w3raw: &W3Raw) -> Self {
        let buffer = String::from_utf8_lossy(&w3raw.data).to_string();
        let re = Regex::new(EXTRACT_DATA).unwrap();
        let mut trigger_strings = HashMap::new();
        for caps in re.captures_iter(buffer.as_str()) {
            let id = caps.get(1).unwrap().as_str().to_string();
            let content = String::from(caps.get(2).unwrap().as_str());
            trigger_strings.insert(id, content);
        }
        WtsFile { trigger_strings }
    }

    pub fn get_ts(&self, id: &str) -> Option<&Trigstr> {
        self.trigger_strings.get(id)
    }
}
