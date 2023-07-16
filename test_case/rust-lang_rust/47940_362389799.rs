rust
macro_rules! test_macro {
    ({$($key:expr => $value:tt),*}) => {};
    ($($key:expr => $value:tt),*) => {};
}

fn test() {
    test_macro!("a" => ("b"));
    test_macro!({"a" => ("b")});
}
