# war3parser

[![Crates.io Version](https://img.shields.io/crates/v/war3parser)](https://crates.io/crates/war3parser)
[![docs.rs](https://img.shields.io/docsrs/war3parser)](https://docs.rs/war3parser)
[![NPM Version](https://img.shields.io/npm/v/%40wesleyel%2Fwar3parser)](https://www.npmjs.com/package/@wesleyel/war3parser)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/wesleyel/war3parser/build.yml)](https://github.com/wesleyel/war3parser/actions/workflows/build.yml)
[![GitHub Release](https://img.shields.io/github/v/release/wesleyel/war3parser)](https://github.com/wesleyel/war3parser/releases)

`war3parser` is a library for parsing and extracting Warcraft III map files. It provides functionality to extract data from MPQ files and supports parsing various file formats.

## Features

- Extract files from MPQ archives
- Support for parsing W3I, WTS, and other file formats
- Command-line tool for file extraction and export

## Usage

### use as a library

```bash
cargo add war3parser
```

```rust
use war3parser::war3map_metadata::War3MapMetadata;

let buffer = std::fs::read("path/to/map.w3x").unwrap();
let metadata = War3MapMetadata::from(&buffer).unwrap();

metadata.save("out");
```

### use as a CLI

```bash
cargo install war3parser-cli
```

```plaintext
$ war3parser-cli help
A extractor and parser for Warcraft 3 map files

Usage: war3parser-cli <COMMAND>

Commands:
  dump-metadata   Dump metadata from a map file [aliases: d]
  extract-file    Extract a file from a MPQ archive and save it [aliases: x]
  extract-images  Extract images with *.tga and *.blp extensions [aliases: i]
  convert-image   Convert a *tga/blp file to png [aliases: c]
  list-files      List files in a MPQ archive [aliases: l]
  help            Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### use as a WASM

```bash
npm install @wesleyel/war3parser
```

```javascript
import { WasmMapInfo } from "@wesleyel/war3parser";

const mapInfo = WasmMapInfo.new(Uint8Array.from(buffer));

console.log(mapInfo);
```

## Contributing

Contributions are welcome! Please submit a Pull Request or report an Issue.

## License

`war3parser` is licensed under the MIT License. See the LICENSE file for details.
