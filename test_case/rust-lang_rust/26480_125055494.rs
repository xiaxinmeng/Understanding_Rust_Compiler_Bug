 rust
macro_rules! cast {
    ($x:expr) => ($x as ())
}

fn main() {
    cast!(2);
}
