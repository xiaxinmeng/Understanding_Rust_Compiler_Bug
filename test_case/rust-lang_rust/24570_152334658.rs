
use std::fs::File;

fn main() {
    let f = File::open("hello.rs");
    println!("{:?}", f);
}
