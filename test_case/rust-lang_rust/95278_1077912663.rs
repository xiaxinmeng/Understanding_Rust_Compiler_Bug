rust
macro_rules! a {
    ($p:path) => {}
}

fn main() {
    a!(Fn()); // OK
}
