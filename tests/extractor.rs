#[cfg(test)]
mod tests {
    use war3parser::extractor::{Extractor, War3Format};

    pub struct MapCase {
        pub filename: &'static str,
        pub path: &'static str,
        pub wts_present: bool,
        pub w3i_present: bool,
        pub mapj_present: bool,
        pub map_preview_present: bool,
        pub map_minimap_present: bool,
        pub listfile_present: bool,
        pub listfile_count: usize,
        pub wts_lines: usize,
    }

    const MAP_1: MapCase = MapCase {
        filename: "(6)BlizzardTD.w3x",
        path: "misc/(6)BlizzardTD.w3x",
        wts_present: true,
        w3i_present: true,
        mapj_present: true,
        map_preview_present: false,
        map_minimap_present: true,
        listfile_present: true,
        listfile_count: 82,
        wts_lines: 5462,
    };

    fn load_mpq(filename: &str) -> Extractor {
        let buf = std::fs::read(filename).expect("failed to read file");
        Extractor::new(&buf)
    }

    #[test]
    fn mpq_list_files() {
        let mut extractor = load_mpq(MAP_1.path);
        let listfiles = extractor.list().expect("failed to list files");
        assert_eq!(listfiles.len(), MAP_1.listfile_count);

        assert_eq!(War3Format::Wts.is_present(&listfiles), MAP_1.wts_present);
        assert_eq!(War3Format::W3i.is_present(&listfiles), MAP_1.w3i_present);
        assert_eq!(War3Format::MapJ.is_present(&listfiles), MAP_1.mapj_present);
        assert_eq!(
            War3Format::MapPreview.is_present(&listfiles),
            MAP_1.map_preview_present
        );
        assert_eq!(
            War3Format::MapMinimap.is_present(&listfiles),
            MAP_1.map_minimap_present
        );
    }

    #[test]
    fn mpq_extract_format() {
        let mut extractor = load_mpq(MAP_1.path);
        let wts = extractor
            .extract(War3Format::Wts)
            .expect("failed to extract wts");

        assert_eq!(wts.filename, "war3map.wts");
        assert_eq!(wts.to_string().lines().count(), MAP_1.wts_lines);
    }
}
