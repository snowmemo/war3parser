use war3parser::extractor::Extractor;

mod tests {
    use war3parser::{extractor::War3Format, parser::w3i::W3iFile};

    use super::*;

    fn load_map() -> Extractor {
        let buf = include_bytes!("../assets/TowerSurvivorsv1.71.w3x");
        Extractor::new(buf)
    }

    #[test]
    fn test_list_file_exists() {
        let mut extractor = load_map();
        let listfiles = extractor.list().expect("failed to list files");
        // for file in listfiles {
        //     println!("{}", file);
        //     if let Some(raw) = extractor.extract_with_filename(&file) {
        //         std::fs::write(format!("assets/test/{}", file), raw.data).unwrap();
        //     }
        // }
        assert_eq!(listfiles.contains(&"war3map.w3i".to_string()), true);
    }

    #[test]
    fn test_extract_file() {
        let mut extractor = load_map();
        let raw = extractor
            .extract(War3Format::W3i)
            .expect("failed to extract w3i");
        let w3i = W3iFile::try_from_w3raw(raw).expect("failed to parse w3i");
        println!("{:?}", w3i.version);
    }
}
