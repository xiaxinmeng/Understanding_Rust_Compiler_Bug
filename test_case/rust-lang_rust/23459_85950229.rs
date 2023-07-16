 rust
use std::fs::File;
use std::path::Path;
// use std::io::Write;

fn main() {
    let mut file = File::create("hello.txt").unwrap();
    write!(&mut file, "Hello, {}", "world!").unwrap();
}
