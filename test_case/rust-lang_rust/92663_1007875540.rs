rust
use std::io::{Cursor, Write};

fn main() {
    let mut writer = Cursor::new([0u8; 128]);
    write!(writer, "...").unwrap();
    let n = writer.position();
    println!("{:?}", &writer.into_inner()[..n as usize]);
}
