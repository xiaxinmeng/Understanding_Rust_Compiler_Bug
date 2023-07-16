 rust
use std::path::MAIN_SEPARATOR;
use std::str;

pub const S: char = '/';

#[no_mangle]
pub extern "C" fn repro() {
    let a: &[u8] = b"123456";
    let mut chars = str::from_utf8(a).unwrap().chars();
    match chars.next() {
        Some(MAIN_SEPARATOR) => println!("Test"),
        _ => println!("Nope"),
    }
}


fn main() {
    println!("Hello, world!");
}

