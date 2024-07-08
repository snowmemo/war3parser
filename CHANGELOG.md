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
