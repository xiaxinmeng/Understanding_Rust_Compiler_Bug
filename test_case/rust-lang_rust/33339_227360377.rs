 rust
#![feature(type_ascription)]

fn main() {
    let x = b"foo".as_ref();
    let y = x : &[u8];
}
