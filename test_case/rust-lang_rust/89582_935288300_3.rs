rust
use std::env;
use std::fs::File;
use std::io::Read;

pub fn main() {
    let mut file = File::open(env::args().nth(1).expect("no file name")).expect("open failed");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("read_to_string failed");
}
