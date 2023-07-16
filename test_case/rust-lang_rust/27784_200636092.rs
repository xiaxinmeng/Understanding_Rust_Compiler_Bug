 rust
#![no_std]
#![crate_type="rlib"]

pub fn main() {
    let mut b = [0];
    'b'.encode_utf8(&mut b);
}
