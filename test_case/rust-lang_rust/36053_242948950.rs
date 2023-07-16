 rust
#![feature(fused)]
use std::iter::FusedIterator;

struct Thing<'a>(&'a str);
impl<'a> Iterator for Thing<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        None
    }
}

//impl<'a> FusedIterator for Thing<'a> {}

fn main() {
    Thing("test").fuse().filter(|_| true).count();
}
