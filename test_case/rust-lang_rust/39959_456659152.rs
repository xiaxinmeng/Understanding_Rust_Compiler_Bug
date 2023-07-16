rust
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;

// Comment this out... and everything will work fine
use thisdoesntexistyolo::haha;

struct Something<T: Read + Write> {
    thing: T,
}

impl<'a, T: 'a + Read + Write> Something<T>
    where &'a T: Read
{
    fn new(thing: T) -> Something<T> {
        Something { thing }
    }
}

fn main() {
    let file = File::open(Path::new("doesntmatter.txt")).unwrap();
    let mut st: Something<File> = Something::new(file);
}
