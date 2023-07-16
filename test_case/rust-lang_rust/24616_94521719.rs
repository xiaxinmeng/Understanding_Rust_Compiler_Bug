 rust
use std::path::Path;
use std::convert::AsRef;
use std::iter::Peekable;
use std::env;

trait PathIterator: Iterator where <Self as Iterator>::Item : AsRef<Path> {}

impl<I: ?Sized, T: AsRef<Path>> PathIterator for I
    where I: Iterator<Item = T> {}

struct A<I: PathIterator> {
    paths: Peekable<I>,
}

impl<I: PathIterator> A<I> {
    fn next_path(&self) -> Option<&Path> {
        self.paths.peek().map(|p| p.as_ref())
    }
}

fn main() { }
