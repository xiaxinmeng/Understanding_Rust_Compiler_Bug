rust
macro_rules! mul {
    ($l:expr, $r:expr) => ($l * $r);
}

mul!(1 + 2, 3 + 4)
