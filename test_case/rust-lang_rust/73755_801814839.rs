rust
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test(indices: Vec<i32>, floats: Vec<f32>, k: i32) -> Option<f32> {
    let mut data: HashMap<_, _> = indices.into_iter().zip(floats.into_iter()).collect();
    data.remove(&k)
}
