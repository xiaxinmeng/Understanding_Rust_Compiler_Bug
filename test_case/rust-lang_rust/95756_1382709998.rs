rust
use std::io::Read;

fn main() {
    let mut reader: &[u8] = b"hello";
    let mut out = [0u8; 16];
    while reader.read(&mut out).unwrap() > 0 {}
    
    println!("{reader:?}"); // []
}
