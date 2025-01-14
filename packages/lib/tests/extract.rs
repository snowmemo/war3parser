use war3parser::war3map_metadata::War3MapMetadata;

fn load_map() -> Option<War3MapMetadata> {
    let buf = include_bytes!("../assets/TowerSurvivorsv1.71.w3x");
    War3MapMetadata::from(buf)
}

#[test]
fn test_w3x_parse() {
    let map = load_map().expect("failed to load map");
    let map_info = map.map_info.expect("failed to get map info");
    let _imp = map.imp.expect("failed to get imports");
    let _wts = map.wts.expect("failed to get string table");
    let _minimap = map.minimap.expect("failed to get minimap");
    let _preview = map.preview.expect("failed to get preview");

    println!("map name: {:?}", map_info.name);
    println!("map author: {:?}", map_info.author);
    println!("map description: {:?}", map_info.description);

    println!(
        "map loading_screen_title: {:?}",
        map_info.loading_screen_title
    );
    println!(
        "map loading_screen_subtitle: {:?}",
        map_info.loading_screen_subtitle
    );
    println!(
        "map loading_screen_text: {:?}",
        map_info.loading_screen_text
    );
}
