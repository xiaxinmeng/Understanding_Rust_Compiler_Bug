
fn main() {
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::fs::File;

    let mut f = File::open("log.txt").unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let len = reader.read_line(&mut line).unwrap();
    println!("First line is {} bytes long", len);
}
