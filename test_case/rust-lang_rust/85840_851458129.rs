rust
use std::fs;

fn repro() {
    let s = fs::read("/absolute/path.txt").unwrap();
    println!("{:#?}", s);
}

fn main() {
    repro();
}
