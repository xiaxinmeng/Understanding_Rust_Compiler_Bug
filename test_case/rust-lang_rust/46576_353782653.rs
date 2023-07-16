
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub fn read_from_file(path: &str) {
    let file = File::open(&path).unwrap();
    let mut reader = BufReader::new(file);
    let mut s = String::new();
    reader.read_to_string(&mut s).unwrap();
    
    // comment out this line to solve bug
    s.lines();
}

fn main() {}
