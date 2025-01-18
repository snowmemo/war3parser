use std::collections::HashMap;

use js_sys::{self, Float32Array, Int32Array, Uint32Array, Uint8Array};
use wasm_bindgen::JsValue;

pub fn u32_array_to_uint32array<const N: usize>(array: [u32; N]) -> Uint32Array {
    let new_array = Uint32Array::new_with_length(N as u32);
    new_array.copy_from(array.as_slice());
    new_array
}

pub fn i32_array_to_int32array<const N: usize>(array: [i32; N]) -> Int32Array {
    let new_array = Int32Array::new_with_length(N as u32);
    new_array.copy_from(array.as_slice());
    new_array
}

pub fn f32_array_to_float32array<const N: usize>(array: [f32; N]) -> Float32Array {
    let new_array = Float32Array::new_with_length(N as u32);
    new_array.copy_from(array.as_slice());
    new_array
}

pub fn u8_array_to_uint8array<const N: usize>(array: [u8; N]) -> Uint8Array {
    let new_array = Uint8Array::new_with_length(N as u32);
    new_array.copy_from(array.as_slice());
    new_array
}

pub fn hashmap_to_js_map<K: Into<JsValue>, V: Into<JsValue>>(map: HashMap<K, V>) -> js_sys::Map {
    let js_map = js_sys::Map::new();
    for (key, value) in map {
        js_map.set(&key.into(), &value.into());
    }
    js_map
}
