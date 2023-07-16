 rust
macro_rules! foo {
    ($($a:tt)*) => ($($a)*)
}
foo! {
    macro_rules! bar { // same as foo
        ($($a:tt)*) => ($($a)*)
    }
}
