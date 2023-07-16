
use std::fs::File;

fn main() {
    let _x = File::create("x").unwrap();
    let _y = File::create("y").unwrap();
}
