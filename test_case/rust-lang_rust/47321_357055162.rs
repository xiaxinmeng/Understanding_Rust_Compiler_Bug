rust
extern crate byteorder;

use std::io::Cursor;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};

fn main() {
    const LEN: usize = 100000000;
    let mut buf: Vec<u8> = Vec::with_capacity(LEN);
    unsafe { buf.set_len(LEN); };

    let mut cur = Cursor::new(&buf);

    let mut b = 0u8;
    loop {
        if let Ok(n) = cur.read_u8() {
            b += n;
        } else {
            break;
        }
    }
    println!("{}", b);
}
