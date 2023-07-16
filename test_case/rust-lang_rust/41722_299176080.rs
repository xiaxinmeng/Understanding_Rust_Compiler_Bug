rust
macro_rules! m {
    ($x: expr) => {};
    (~ $x: expr) => {};
}

fn main() {
    m!(~x); // ERROR expected expression, found `~`
}
