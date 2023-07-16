 rust
#![feature(rand)]

fn main() {
    extern crate rand;
    let rng = rand::chacha::ChaChaRng::new_unseeded();
}
