rust
macro_rules! check {
    ($e:expr) => {};
    (NOT $a:literal) => {};
}

check! { NOT 1 }
