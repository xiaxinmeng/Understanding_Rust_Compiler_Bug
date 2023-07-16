 rust
use std::io::BufRead;

fn main() {
    let bar = std::io::stdin();
    let foo: String = bar.lock().lines().nth(0).unwrap().unwrap();
}
