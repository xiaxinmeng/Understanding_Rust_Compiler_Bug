 rust
macro_rules! foo {
    ($x:tt) => (type t = $x<i32>;)
}

foo!(Box);
