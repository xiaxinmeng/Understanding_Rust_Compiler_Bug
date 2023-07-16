rust
macro_rules! mypanic {
    ($($t:tt)*) => { assert!(false, $($t:tt)*) };
}
