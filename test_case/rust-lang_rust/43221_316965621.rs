rust
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

fn main() {
    let f = File::open("I:\\home\\projects\\test\\rust\\UTF-8-demo.txt").unwrap();
    let mut file = BufReader::new(&f);
    for line in file.lines() {
        let line = line.unwrap();
        let slice : &str = &line;
        let slice2 = slice;

        println!("{}", slice);
    }
}
