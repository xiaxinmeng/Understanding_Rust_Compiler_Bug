rust
#[cfg(test)]
extern crate wasm_bindgen_test;
#[cfg(test)]
use wasm_bindgen_test::*;

#[cfg(test)]
#[wasm_bindgen_test]
pub fn recv() {
    let (tx, rx) = std::sync::mpsc::channel::<()>();
    let _ = tx.clone();
    let _ = rx.recv();
}
