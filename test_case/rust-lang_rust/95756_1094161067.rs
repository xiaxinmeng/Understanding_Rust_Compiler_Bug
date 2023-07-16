rust
use std::io;

fn main() {
    let reader: &[u8] = b"hello";
    let mut writer: Vec<u8> = vec![];

    io::copy(&mut {reader}, &mut writer).unwrap();

    println!("reader: {:?}", &reader);
    println!("writer: {:?}", &writer);
}
