Rust
#![feature(unsized_locals, raw_ref_op)]

fn main() {
    let f: fn([u8]) = |_| {};
}
