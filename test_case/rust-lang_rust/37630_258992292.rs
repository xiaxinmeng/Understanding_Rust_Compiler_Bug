 rust
use std::io::{self, BufRead};

fn main() {
    let inp = io::stdin();
    let mut inp = inp.lock();

    let mut line = String::new();

    inp.read_line(&mut line).unwrap();

    println!("{}", line.trim().parse::<f64>().unwrap());
}
