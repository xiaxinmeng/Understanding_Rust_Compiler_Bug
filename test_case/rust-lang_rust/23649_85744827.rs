 rust
pub struct X([u8]);

fn _f(x: &X) { match *x { X(ref x) =>  { } } }

fn main() { }
