rust
macro_rules! foo {
    ($tt1:tt) => {};
}

fn x() {
    foo!('me#foo);
}
