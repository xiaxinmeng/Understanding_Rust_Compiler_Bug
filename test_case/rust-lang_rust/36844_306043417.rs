rust
macro_rules! a {
    () => { a!() }
}

fn main() {
    a!()
}
