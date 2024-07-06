use war3parser::extractor::{Extractor, War3Format};
pub struct MapCase {
    pub path: &'static str,
    pub wts_present: bool,
    pub w3i_present: bool,
    pub mapj_present: bool,
    pub map_preview_present: bool,
    pub map_minimap_present: bool,
    pub listfile_count: usize,
    pub wts_lines: usize,
}

const MAP_1: MapCase = MapCase {
    path: "misc/(6)BlizzardTD.w3x",
    wts_present: true,
    w3i_present: true,
    mapj_present: true,
    map_preview_present: false,
    map_minimap_present: true,
    listfile_count: 82,
    wts_lines: 5462,
};

pub fn load_mpq(filename: &str) -> Extractor {
    let buf = std::fs::read(filename).expect("failed to read file");
    Extractor::new(&buf)
}

mod tests {
    use image::RgbaImage;
    use war3parser::{parser::w3i::W3iFile, preview::ImageRaw};

    use super::*;

    #[test]
    fn test_list_files() {
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
    fn test_extract_format() {
        let mut extractor = load_mpq(MAP_1.path);
        let wts = extractor
            .extract(War3Format::Wts)
            .expect("failed to extract wts");

        assert_eq!(wts.filename, "war3map.wts");
        assert_eq!(wts.to_string().lines().count(), MAP_1.wts_lines);
    }

    #[test]
    fn test_blp_image() {
        let mut extractor = load_mpq(MAP_1.path);
        let blp = extractor
            .extract(War3Format::MapMinimap)
            .expect("failed to extract minimap");

        let img = ImageRaw::from_w3raw(blp);
        let rgba: RgbaImage = img.try_into().expect("failed to convert to rgba");

        let expected = image::open("misc/minimap.png")
            .expect("failed to load image")
            .into_rgba8();
        assert_eq!(rgba.dimensions(), expected.dimensions());
        assert_eq!(
            rgba.pixels().collect::<Vec<_>>(),
            expected.pixels().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_tga_image() {
        let mut extractor = load_mpq(MAP_1.path);
        let tga = extractor
            .extract_with_filename("war3mapImported\\PASBTNSlow.tga")
            .expect("failed to extract minimap");

        let img = ImageRaw::from_w3raw(tga);
        let rgba: RgbaImage = img.try_into().expect("failed to convert to rgba");

        let expected: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::open("misc/slow.png")
            .expect("failed to load image")
            .into_rgba8();
        assert_eq!(rgba.dimensions(), expected.dimensions());
        assert_eq!(
            rgba.pixels().collect::<Vec<_>>(),
            expected.pixels().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_parse_w3i_tft() {
        let mut extractor = load_mpq(MAP_1.path);
        let w3i = extractor
            .extract(War3Format::W3i)
            .expect("failed to extract w3i");

        assert_eq!(w3i.filename, "war3map.w3i");
        let w3i: W3iFile = w3i.try_into().unwrap();
        assert_eq!(w3i.map_width, 160);
        assert_eq!(w3i.map_height, 128);
    }
}
