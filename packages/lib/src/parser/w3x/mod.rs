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

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: RgbaImage,
}

pub struct War3MapW3x {
    pub u1: Option<u32>,
    pub name: Option<String>,
    pub flags: Option<u32>,
    pub max_players: Option<u32>,

    pub archive: Archive,
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
    pub fn get_file_names(archive: &mut Archive) -> Result<Vec<String>, ParserError> {
        let file = archive.open_file("(listfile)")?;
        let mut data = vec![0; file.size() as usize];
        file.read(archive, &mut data).unwrap();
        let listfile = String::from_utf8(data)?;
        Ok(listfile.split("\r\n").map(|s| s.to_string()).collect())
    }

    pub fn get(&mut self, filename: &str) -> Result<File, ParserError> {
        self.archive.open_file(filename).map_err(ParserError::from)
    }

    pub fn has(&mut self, filename: &str) -> bool {
        self.archive.open_file(filename).is_ok()
    }

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

    pub fn read_map_info(&mut self) -> Result<War3MapW3i, ParserError> {
        let file = self.get("war3map.w3i")?;
        let mut data = vec![0; file.size() as usize];
        file.read(&mut self.archive, &mut data)?;

        let mut reader = BinaryReader::from_vec(&data);
        reader.set_endian(Endian::Little);

        let map_info = War3MapW3i::load(&mut reader, 0)?;
        Ok(map_info)
    }

    pub fn read_imports(&mut self) -> Result<War3MapImp, ParserError> {
        let file = self.get("war3map.imp")?;
        let mut data = vec![0; file.size() as usize];
        file.read(&mut self.archive, &mut data)?;
        let mut reader = BinaryReader::from_vec(&data);
        reader.set_endian(Endian::Little);

        War3MapImp::load(&mut reader, 0)
    }

    pub fn read_string_table(&mut self) -> Result<War3MapWts, ParserError> {
        let file = self.get("war3map.wts")?;
        let mut data = vec![0; file.size() as usize];
        file.read(&mut self.archive, &mut data)?;
        let buffer = String::from_utf8(data)?;
        War3MapWts::load(&buffer)
    }

    fn buffer_to_image(data: &[u8]) -> Result<Image, ParserError> {
        if BlpImage::is_blp(&data) {
            let blp = BlpImage::load(&data)?;
            Ok(Image {
                width: blp.width,
                height: blp.height,
                data: blp.data,
            })
        } else if TgaImage::is_tga(&data) {
            let tga = TgaImage::load(&data)?;
            Ok(Image {
                width: tga.width,
                height: tga.height,
                data: tga.data,
            })
        } else {
            Err(ParserError::FailedToFindMinimap)
        }
    }

    pub fn read_minimap(&mut self) -> Result<Image, ParserError> {
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

    pub fn read_preview(&mut self) -> Result<Image, ParserError> {
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
