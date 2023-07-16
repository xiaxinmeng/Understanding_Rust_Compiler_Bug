rust
macro_rules! check {
    ($e:expr) => {};
    (not $a:literal) => {};
}

check! { not 1 }
