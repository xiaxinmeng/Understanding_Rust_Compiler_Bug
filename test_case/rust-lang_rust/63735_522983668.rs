rust
use std::path::Path;

fn test<P>(_: Option<P>) where P: AsRef<Path> + Default {
    let instance = P::default();
}

fn main() {
    test(None);
}
