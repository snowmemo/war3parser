/// Binary reader to read binary data
pub mod binary_reader;

/// Custom error types
pub mod error;

/// [`crate::parser::imp::War3MapImp`] parser for `war3map.imp` file
pub mod imp;

/// [`crate::parser::w3i::War3MapW3i`] parser for `war3map.w3i` file
pub mod w3i;

/// [`crate::parser::wts::War3MapWts`] parser for `war3map.wts` file
pub mod wts;

/// [`crate::parser::w3x::War3MapW3x`] parser
pub mod w3x;

/// [`crate::parser::tga::TgaImage`] parser for TGA images
pub mod tga;

/// [`crate::parser::blp::BlpImage`] parser for BLP images
pub mod blp;
