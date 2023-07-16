 rust
use std::io::{Cursor,Read};

fn main() {
        let arr : [u8; 4] = [0xff, 0xee, 0xdd, 0xcc];
        let mut c = Cursor::new(&arr as &[u8]);
        let mut arr2 = [0u8; 4];
        let _ = c.read(&mut arr2[0..2]);
        let _ = c.read(&mut arr2[2..4]);
        assert!(arr == arr2);
}
