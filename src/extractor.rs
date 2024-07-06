use std::fmt::{Display, Formatter};

use mpq::Archive;

pub enum War3Format {
    W3i,
    Wts,
    MapJ,
    MapPreview,
    MapMinimap,
    Listfile,
}

impl War3Format {
    fn optional_files(&self) -> Vec<&str> {
        match self {
            War3Format::W3i => vec!["war3map.w3i"],
            War3Format::Wts => vec!["war3map.wts"],
            War3Format::MapJ => vec!["war3map.j", "Scripts/war3map.j", "scripts/war3map.j"],
            War3Format::MapPreview => vec!["war3mapPreview.tga", "war3mapPreview.blp"],
            War3Format::MapMinimap => vec!["war3mapMap.tga", "war3mapMap.blp"],
            War3Format::Listfile => vec!["(listfile)"],
        }
    }

    pub fn is_present(&self, list: &Vec<String>) -> bool {
        let files = self.optional_files();
        for file in files {
            if list.contains(&file.to_string()) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
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

pub struct Extractor {
    archive: Archive,
}
impl Extractor {
    pub fn new(buf: &[u8]) -> Self {
        let buf_vec = buf.to_vec();
        let archive = Archive::load(buf_vec).unwrap();
        Extractor { archive }
    }

    /// extract formats by filename
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

    /// extract formats by formats with fallbacks
    pub fn extract(&mut self, format: War3Format) -> Option<W3Raw> {
        for file in format.optional_files() {
            if let Some(content) = self.extract_with_filename(file) {
                return Some(content);
            }
        }
        None
    }

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
}
