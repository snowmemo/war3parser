## 0.2.2 (2024-07-11)

### Feat

- **wasm**: add extractor test
- **cli**: add war3parser-cli tool to extract MPQ file
- **wasm**: TODO: wasm
- **wasm**: init wasm project
- **parser**: add w3 color style string parser
- **parser**: enhanced w3i update_with_hashmap function
- **parser**: add wts parser and w3i update function
- **parser**: add update_with_wts for W3iFile

### Refactor

- **bindgen**: remove bindgen function, use default test gen method
- **parser**: not export bindings at compile time
- change src structure and create war3parser-wasm project

## 0.2.1 (2024-07-08)

### Feat

- **bindings**: add `ts-rs` to export TS bindings for w3 struct
- **parser**: use enum_display to derive Display for enum
- **parser**: add Tileset and MapSize type

### Refactor

- **struct**: remove bitfield-struct, replaced by native struct
- **struct**: use bitfield-struct instead of bitfield
- **parser**: extract war3 file format struct and w3parser trait to separate file
- **parser**: use PartialEq and PartialOrd instead of `gt` `le`

## 0.2.0 (2024-07-06)

### Feat

- **parser**: add w3i file parser
- **parser**: add war3 w3i file structure
- **preview**: add preview module
- init war3parser project
