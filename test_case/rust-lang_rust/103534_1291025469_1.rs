rust
macro_rules! blah {
    ($expr:expr) => {};
    (let) => {};
}

fn main() {
    blah!(not 1);
}
