 rust
macro_rules! foo {
    ($x:tt) => (type t = $x<int>;)
}

foo!(Box);
