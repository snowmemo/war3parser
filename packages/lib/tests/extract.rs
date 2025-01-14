use binary_reader::{BinaryReader, Endian};
use war3parser::parser::{binary_reader::BinaryReadable, w3x::War3MapW3x};

fn load_map() -> &'static [u8] {
    let buf = include_bytes!("../assets/Legion_TD_11.1c_TeamOZE.w3x");
    buf
}

#[test]
fn test_w3x_parse() {
    let mut binary_reader = BinaryReader::from_u8(load_map());
    binary_reader.set_endian(Endian::Little);

    let w3x = War3MapW3x::load(&mut binary_reader, 0).expect("failed to load w3x");
    let mut w3x_box = Box::new(w3x);

    eprintln!("w3x files: {:?}", w3x_box.files);

    let map_info = w3x_box.read_map_info().expect("failed to read map info");
    eprintln!("map info: {:#?}", map_info);
}
