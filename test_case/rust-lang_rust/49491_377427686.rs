Rust
use std::fs::File;
use std::io::{self, prelude::*};

fn run() -> io::Result<File> {
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello, world!")?;
    
    Ok(file)
}

fn main() {
    run().unwrap();
}
