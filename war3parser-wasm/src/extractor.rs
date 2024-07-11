use war3parser::extractor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Extractor(extractor::Extractor);

#[wasm_bindgen]
pub struct W3Raw(extractor::W3Raw);

#[wasm_bindgen]
pub struct War3Format(extractor::War3Format);

#[wasm_bindgen]
impl War3Format {
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> War3Format {
        War3Format(extractor::War3Format::from(name))
    }
}

#[wasm_bindgen]
pub fn create_extractor(buf: &[u8]) -> Extractor {
    use std::panic;
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    Extractor(extractor::Extractor::new(buf))
}

#[wasm_bindgen]
impl Extractor {
    pub fn extract_with_filename(&mut self, filename: &str) -> Option<W3Raw> {
        self.0.extract_with_filename(filename).map(W3Raw)
    }

    pub fn extract(&mut self, format: War3Format) -> Option<W3Raw> {
        self.0.extract(format.0).map(W3Raw)
    }
}

#[wasm_bindgen]
impl W3Raw {
    pub fn filename(&self) -> String {
        self.0.filename.clone()
    }

    pub fn data(&self) -> Vec<u8> {
        self.0.data.clone()
    }

    pub fn size(&self) -> usize {
        self.0.size
    }
}
