rust
// build-pass
// ignore-tidy-linelength
// ignore-wasm32-bare compiled with panic=abort by default
// compile-flags: -Z mir-opt-level=3
#![feature(box_syntax)]

fn main() {
    let _x: Box<Vec<u32>> = box Vec::new();
}
