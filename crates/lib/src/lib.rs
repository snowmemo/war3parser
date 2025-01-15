//! # War3Parser
//!
//! `war3parser` is a library for parsing and extracting Warcraft III map files.
//!
//! ## Features
//!
//! - Extract files from MPQ archives. _(As long as the file name is known and the file exists in the MPQ)_
//! - Parse file formats into Rust structs
//!
//! ### Supported parse targets
//!
//! - W3X: [`War3MapW3x`]
//! - W3I: [`War3MapW3i`]
//! - WTS: [`War3MapWts`]
//! - BLP: [`BlpImage`]
//! - TGA: [`TgaImage`]
//!
//! And a helper struct to include all supported metadata of a map file: [`War3MapMetadata`]
//!
//! ### Implementation
//!
//! Most of the struct implemented [`BinaryReadable`] trait, which provides a `load` function to load the struct from a binary reader.
//!
//! We use the trait implementation chain to load the struct from a file.

/// Parsers to do the actual parsing
pub mod parser;

/// Helper struct that includes all supported metadata of a map file
pub mod war3map_metadata;

#[doc(inline)]
pub use war3map_metadata::War3MapMetadata;

#[doc(inline)]
pub use parser::{
    error::ParserError,
    imp::War3MapImp,
    w3i::War3MapW3i,
    w3x::{War3Image, War3MapW3x},
    wts::War3MapWts,
    binary_reader::BinaryReadable,
    blp::BlpImage,
    tga::TgaImage,
};
