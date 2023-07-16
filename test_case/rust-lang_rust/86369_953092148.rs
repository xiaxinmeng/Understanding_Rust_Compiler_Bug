rust
#![feature(cursor_remaining)]

use std::io::Write;

fn main() {
    let mut buf = [0u8; 10];
    let mut cursor = std::io::Cursor::new(&mut buf[..]);
    dbg!(cursor.is_empty()); // false
    cursor.write_all(b"1234567").unwrap();
    dbg!(cursor.is_empty()); // false
    cursor.write_all(b"890").unwrap();
    dbg!(cursor.is_empty()); // true
}
