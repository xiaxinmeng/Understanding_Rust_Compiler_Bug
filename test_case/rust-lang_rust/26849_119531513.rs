 rust
// based on @AlisdairO's test
use std::env;
use std::fs;
use std::io::Read;

fn main() {
    let arg = env::args().nth(1).unwrap();
    let mut f = fs::File::open(arg).unwrap();
    let mut data = Vec::with_capacity(1024);
    f.read_to_end(&mut data).unwrap();
    println!("{}", data.len());
}
