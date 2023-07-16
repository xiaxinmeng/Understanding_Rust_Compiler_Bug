
use std::fs::File;

fn main() {
    println!("{:?}", File::create("/tmp/\0"));
}
