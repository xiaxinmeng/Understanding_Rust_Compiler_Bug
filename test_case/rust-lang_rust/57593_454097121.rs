rust
macro_rules! my_macro {
    (one_arg $e:expr) => { ... },
    (two_arg $e:expr, $e:expr) => { ... },
}

my_macro!(one_arg, 5);
