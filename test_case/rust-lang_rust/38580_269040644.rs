rust
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let sum: Result<usize, _> = stdin.lock().lines().map(|l| {
        l.map(|l| l.len())
    }).sum();
    println!("{:?}", sum);
}
