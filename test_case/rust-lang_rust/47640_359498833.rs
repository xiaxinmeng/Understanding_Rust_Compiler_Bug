rust
use std::io::{Cursor, Read};

fn main() {
    let mut io = Cursor::new(&[1, 2][..]);
    assert!(io.read_exact(&mut [0; 4]).is_err());
    println!("{}", io.position());
}
