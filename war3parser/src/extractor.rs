use std::fmt::{Display, Formatter};

use mpq::Archive;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::parser::{w3i::W3iFile, wts::WtsFile};

/// Pre-defined filenames of MPQ files
#[derive(Debug)]
#[wasm_bindgen]
pub enum War3Format {
    W3i,
    Wts,
    Wtg,
    MapJ,
    MapPreview,
    MapMinimap,
    Listfile,
}

impl From<&str> for War3Format {
    fn from(s: &str) -> Self {
        match s {
            "w3i" => War3Format::W3i,
            "wts" => War3Format::Wts,
            "wtg" => War3Format::Wtg,
            "map.j" => War3Format::MapJ,
            "mapPreview" => War3Format::MapPreview,
            "mapMinimap" => War3Format::MapMinimap,
            _ => War3Format::Listfile,
        }
    }
}

impl War3Format {
    fn optional_files(&self) -> Vec<&str> {
        match self {
            War3Format::W3i => vec!["war3map.w3i"],
            War3Format::Wts => vec!["war3map.wts"],
            War3Format::Wtg => vec!["war3map.wtg"],
            War3Format::MapJ => vec!["war3map.j", "Scripts/war3map.j", "scripts/war3map.j"],
            War3Format::MapPreview => vec!["war3mapPreview.tga", "war3mapPreview.blp"],
            War3Format::MapMinimap => vec!["war3mapMap.tga", "war3mapMap.blp"],
            War3Format::Listfile => vec!["(listfile)"],
        }
    }

    pub fn is_present(&self, list: &[String]) -> bool {
        let files = self.optional_files();
        for file in files {
            if list.contains(&file.to_string()) {
                return true;
            }
        }
        false
    }
}

/// Raw data of a file extracted from the MPQ archive
///
/// - `filename`: The filename of the file
/// - `data`: The raw data of the file
/// - `size`: The size of the file
#[derive(Debug)]
#[wasm_bindgen(getter_with_clone)]
pub struct W3Raw {
    pub filename: String,
    pub data: Vec<u8>,
    pub size: usize,
}

impl Display for W3Raw {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match String::from_utf8(self.data.clone()) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(std::fmt::Error),
        }
    }
}

impl W3Raw {
    pub fn save(&self, filename: &str) {
        std::fs::write(filename, &self.data).unwrap();
    }
}

/// Extractor of MPQ files
#[wasm_bindgen]
pub struct Extractor {
    archive: Archive,
}

#[wasm_bindgen]
impl Extractor {
    /// Create a new extractor from a buffer
    pub fn new(buf: &[u8]) -> Self {
        let buf_vec = buf.to_vec();
        let archive = Archive::load(buf_vec).unwrap();
        Extractor { archive }
    }

    /// Extract a file from the MPQ archive by filename
    pub fn extract_with_filename(&mut self, filename: &str) -> Option<W3Raw> {
        match self.archive.open_file(filename) {
            Ok(file) => {
                let mut data = vec![0; file.size() as usize];
                file.read(&mut self.archive, &mut data).unwrap();
                Some(W3Raw {
                    filename: filename.to_string(),
                    data,
                    size: file.size() as usize,
                })
            }
            _ => None,
        }
    }

    /// Extract a file from the MPQ archive by format
    pub fn extract(&mut self, format: War3Format) -> Option<W3Raw> {
        for file in format.optional_files() {
            if let Some(content) = self.extract_with_filename(file) {
                return Some(content);
            }
        }
        None
    }

    /// List all files in the MPQ archive
    pub fn list(&mut self) -> Option<Vec<String>> {
        match self.extract(War3Format::Listfile) {
            Some(content) => {
                let listfile = content.to_string();
                let list: Vec<String> = listfile.split("\r\n").map(|s| s.to_string()).collect();
                Some(list)
            }
            None => None,
        }
    }

    pub fn map_info(&mut self) -> Option<W3iFile> {
        let w3i = self.extract(War3Format::W3i).unwrap();
        let w3i_file = W3iFile::try_from_w3raw(w3i).unwrap();
        let wts = self.extract(War3Format::Wts).unwrap();
        let wts_file: WtsFile = wts.try_into().unwrap();
        let w3i_file = w3i_file.update_with_wts(&wts_file);
        Some(w3i_file)
    }
}
