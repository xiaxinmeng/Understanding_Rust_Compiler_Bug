rust
use std::io::{self, Read, Seek};
use std::fs::File;

fn main() {
    let mut file = File::create("/tmp/ebadf").unwrap();
    let mut data = b"Hello" as &[u8];
    io::copy(&mut data, &mut file).unwrap();
    file.seek(io::SeekFrom::Start(0)).unwrap();
    let mut vec = Vec::new();
    file.read_to_end(&mut vec).unwrap();
}
