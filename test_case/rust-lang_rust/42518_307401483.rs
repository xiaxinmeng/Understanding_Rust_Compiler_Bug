rust
use std::fs::File;
use std::io::{Seek, SeekFrom};

fn main() {
    let mut f = File::open("x.txt").unwrap();
    f.set_len(5).unwrap();
    f.seek(SeekFrom::Start(2)).unwrap();
}
