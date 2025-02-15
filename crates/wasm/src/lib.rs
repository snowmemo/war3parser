//! WASM bindings for war3parser
//!
//! [![NPM Version](https://img.shields.io/npm/v/%40wesleyel%2Fwar3parser)](https://www.npmjs.com/package/@wesleyel/war3parser)
//!
//! This crate is used to create WASM bindings for the war3parser crate.
//!
//! Prebuilt wasm files are available in [@wesleyel/war3parser](https://www.npmjs.com/package/@wesleyel/war3parser)
pub mod impls;
pub mod types;

#[doc(inline)]
pub use types::*;

#[doc(inline)]
pub use impls::*;
