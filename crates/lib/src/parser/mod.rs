/// Binary reader to read binary data
pub mod binary_reader;

/// Custom error types
pub mod error;

/// [`War3MapImp`] parser for `war3map.imp` file
pub mod imp;

/// [`War3MapW3i`] parser for `war3map.w3i` file
pub mod w3i;

/// [`War3MapWts`] parser for `war3map.wts` file
pub mod wts;

/// [`War3MapW3x`] parser
pub mod w3x;

/// [`TgaImage`] parser for TGA images
pub mod tga;

/// [`BlpImage`] parser for BLP images
pub mod blp;

#[doc(inline)]
pub use {
    imp::War3MapImp,
    w3i::War3MapW3i,
    wts::War3MapWts,
    w3x::War3MapW3x,
    tga::TgaImage,
    blp::BlpImage,
    binary_reader::BinaryReadable,
    error::ParserError,
};
