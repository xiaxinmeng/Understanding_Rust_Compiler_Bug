rust
macro_rules! a {
    () => (
        b!( () )
    );
}
macro_rules! b {
    ($expr:expr) => (
        if true $expr
    );
}

pub fn main() {
    a!();
}
