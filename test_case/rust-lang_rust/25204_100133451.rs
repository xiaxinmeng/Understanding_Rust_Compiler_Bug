 rust
fn weird<T: ?Sized>(_: Option<T>) where Option<T>: Sized {}

fn main() {}
