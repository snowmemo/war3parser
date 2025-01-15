use std::path::PathBuf;

use binary_reader::{BinaryReader, Endian};
use image::RgbaImage;
use mpq::{Archive, File};

use super::{
    binary_reader::{AutoReadable, BinaryReadable},
    blp::BlpImage,
    error::ParserError,
    imp::War3MapImp,
    tga::TgaImage,
    w3i::War3MapW3i,
    wts::War3MapWts,
};

/// Image with basic metadata and [`image::RgbaImage`] data
pub struct War3Image {
    pub width: u32,
    pub height: u32,
    pub data: RgbaImage,
}

/// Warcraft 3 map entry
pub struct War3MapW3x {
    pub u1: Option<u32>,
    pub name: Option<String>,
    pub flags: Option<u32>,
    pub max_players: Option<u32>,

    /// MPQ archive
    pub archive: Archive,
    /// List of files in `(listfile)`
    pub files: Option<Vec<String>>,
}

impl BinaryReadable for War3MapW3x {
    fn load(stream: &mut BinaryReader, _version: u32) -> Result<Self, ParserError> {
        let header: [u8; 4] = AutoReadable::read(stream)?;

        let (u1, name, flags, max_players) = if header != ['H', 'M', '3', 'W'].map(|c| c as u8) {
            (None, None, None, None)
        } else {
            let u1 = AutoReadable::read(stream)?;
            let name = AutoReadable::read(stream)?;
            let flags = AutoReadable::read(stream)?;
            let max_players = AutoReadable::read(stream)?;
            (Some(u1), Some(name), Some(flags), Some(max_players))
        };

        let mut archive = Archive::load(stream.data.clone())?;
        let files = Self::get_file_names(&mut archive).ok();
        Ok(Self {
            u1,
            name,
            flags,
            max_players,
            archive,
            files,
        })
    }
}

impl War3MapW3x {
    /// Load a MPQ file from a path
    pub fn new(path: PathBuf) -> Result<Self, ParserError> {
        let buffer = std::fs::read(path)?;
        Self::from_buffer(&buffer)
    }

    /// Load a MPQ file from a buffer
    pub fn from_buffer(buffer: &[u8]) -> Result<Self, ParserError> {
        let mut binary_reader = BinaryReader::from_u8(buffer);
        binary_reader.set_endian(Endian::Little);
        War3MapW3x::load(&mut binary_reader, 0)
    }

    /// Get list of files in `(listfile)`
    pub fn get_file_names(archive: &mut Archive) -> Result<Vec<String>, ParserError> {
        let file = archive.open_file("(listfile)")?;
        let mut data = vec![0; file.size() as usize];
        file.read(archive, &mut data).unwrap();
        let listfile = String::from_utf8(data)?;
        Ok(listfile.split("\r\n").map(|s| s.to_string()).collect())
    }

    /// Get a file from the MPQ archive
    pub fn get(&mut self, filename: &str) -> Result<File, ParserError> {
        self.archive.open_file(filename).map_err(ParserError::from)
    }

    /// Check if a file exists in the MPQ archive
    pub fn has(&mut self, filename: &str) -> bool {
        self.archive.open_file(filename).is_ok()
    }

    /// Get the script file from the MPQ archive
    pub fn get_script_file(&mut self) -> Option<File> {
        [
            "war3map.j",
            "scripts\\war3map.j",
            "war3map.lua",
            "scripts\\war3map.lua",
        ]
        .iter()
        .find_map(|&filename| self.get(filename).ok())
    }

    /// Read the `w3i` map info from the MPQ archive
    pub fn read_map_info(&mut self) -> Result<War3MapW3i, ParserError> {
        let file = self.get("war3map.w3i")?;
        let mut data = vec![0; file.size() as usize];
        file.read(&mut self.archive, &mut data)?;

        let mut reader = BinaryReader::from_vec(&data);
        reader.set_endian(Endian::Little);

        let map_info = War3MapW3i::load(&mut reader, 0)?;
        Ok(map_info)
    }

    /// Read the `imp` map imports from the MPQ archive
    pub fn read_imports(&mut self) -> Result<War3MapImp, ParserError> {
        let file = self.get("war3map.imp")?;
        let mut data = vec![0; file.size() as usize];
        file.read(&mut self.archive, &mut data)?;
        let mut reader = BinaryReader::from_vec(&data);
        reader.set_endian(Endian::Little);

        War3MapImp::load(&mut reader, 0)
    }

    /// Read the `wts` string table from the MPQ archive
    pub fn read_string_table(&mut self) -> Result<War3MapWts, ParserError> {
        let file = self.get("war3map.wts")?;
        let mut data = vec![0; file.size() as usize];
        file.read(&mut self.archive, &mut data)?;
        let buffer = String::from_utf8(data)?;
        War3MapWts::load(&buffer)
    }

    /// Convert a raw binary buffer to an [`War3Image`]
    fn buffer_to_image(data: &[u8]) -> Result<War3Image, ParserError> {
        if BlpImage::is_blp(&data) {
            let blp = BlpImage::load(&data)?;
            Ok(War3Image {
                width: blp.width,
                height: blp.height,
                data: blp.data,
            })
        } else if TgaImage::is_tga(&data) {
            let tga = TgaImage::load(&data)?;
            Ok(War3Image {
                width: tga.width,
                height: tga.height,
                data: tga.data,
            })
        } else {
            Err(ParserError::FailedToFindMinimap)
        }
    }

    /// Read the minimap from the MPQ archive
    pub fn read_minimap(&mut self) -> Result<War3Image, ParserError> {
        let buffer = [
            "war3mapMap.tga",
            "war3mapMap.blp",
            "war3mappreview.tga",
            "war3mappreview.blp",
        ]
        .iter()
        .find_map(|&filename| self.get(filename).ok())
        .ok_or(ParserError::MapFileNotFound("war3mapMap".to_string()))?;
        let mut data = vec![0; buffer.size() as usize];
        buffer.read(&mut self.archive, &mut data)?;
        Self::buffer_to_image(&data)
    }

    /// Read the preview image from the MPQ archive
    pub fn read_preview(&mut self) -> Result<War3Image, ParserError> {
        let buffer = [
            "war3mapPreview.tga",
            "war3mapPreview.blp",
            "war3mappreview.tga",
            "war3mappreview.blp",
        ]
        .iter()
        .find_map(|&filename| self.get(filename).ok())
        .ok_or(ParserError::MapFileNotFound("war3mapPreview".to_string()))?;
        let mut data = vec![0; buffer.size() as usize];
        buffer.read(&mut self.archive, &mut data)?;
        Self::buffer_to_image(&data)
    }
}
