rust 

use std::io::{stdin, Read};

fn main() {
    let mut input = Vec::new();
    input.reserve(100);
    stdin().read_to_end(&mut input).unwrap();

    println!("{:?}", input);
}
