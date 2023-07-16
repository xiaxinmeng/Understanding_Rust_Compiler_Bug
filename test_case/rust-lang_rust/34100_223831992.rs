 rust
use std::io::{Read, BufRead, BufReader};

fn fun<R: Read>(input: R) {
    let buf_reader = BufReader::new(input);
    let buffer = Vec::new();
    let _ = buf_reader.read_line(&mut buffer);
}

fn main() {}
