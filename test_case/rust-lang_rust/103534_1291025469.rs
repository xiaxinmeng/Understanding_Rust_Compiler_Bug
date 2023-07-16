rust
macro_rules! blah {
    ($expr:expr) => {};
    (not $expr:expr) => {};
}

fn main() {
    blah!(not 1);
}
