use binary_reader::BinaryReader;
use mpq::{Archive, File};

use super::{
    binary_reader::{AutoReadable, BinaryReadable},
    error::ParserError,
    w3i::War3MapW3i,
    wts::War3MapWts,
};

pub struct War3MapW3x {
    pub u1: Option<u32>,
    pub name: Option<String>,
    pub flags: Option<u32>,
    pub max_players: Option<u32>,

    pub archive: Archive,
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

        let archive = Archive::load(stream.data.clone())?;
        Ok(Self {
            u1,
            name,
            flags,
            max_players,
            archive,
        })
    }
}

impl War3MapW3x {
    fn get_file_names(&mut self) -> Result<Vec<String>, ParserError> {
        let file = self.archive.open_file("(listfile)")?;
        let mut data = vec![0; file.size() as usize];
        file.read(&mut self.archive, &mut data).unwrap();
        let listfile = String::from_utf8(data)?;
        Ok(listfile.split("\r\n").map(|s| s.to_string()).collect())
    }

    fn get(&mut self, filename: &str) -> Result<File, ParserError> {
        self.archive.open_file(filename).map_err(ParserError::from)
    }

    fn has(&mut self, filename: &str) -> bool {
        self.archive.open_file(filename).is_ok()
    }

    fn get_script_file(&mut self) -> Option<File> {
        ["war3map.j", "scripts\\war3map.j", "war3map.lua"]
            .iter()
            .find_map(|&filename| self.get(filename).ok())
    }

    fn get_map_info(&mut self) -> Result<War3MapW3i, ParserError> {
        let file = self.get("war3map.w3i")?;
        let mut data = vec![0; file.size() as usize];
        file.read(&mut self.archive, &mut data)?;

        let mut reader = BinaryReader::from_vec(&data);
        let map_info = War3MapW3i::load(&mut reader, 0)?;
        Ok(map_info)
    }

    pub fn read_imports(&mut self) -> Result<Vec<String>, ParserError> {
        unimplemented!()
    }

    pub fn read_string_table(&mut self) -> Result<War3MapWts, ParserError> {
        let file = self.get("war3map.wts")?;
        let mut data = vec![0; file.size() as usize];
        file.read(&mut self.archive, &mut data)?;
        let buffer = String::from_utf8(data)?;
        War3MapWts::load(&buffer)
    }
}
