Rust
#![feature(unsized_locals)]

struct Bug {
    V1: [(); {
        let f: [u8];
        f.len()
    }],
}

fn main() {}
