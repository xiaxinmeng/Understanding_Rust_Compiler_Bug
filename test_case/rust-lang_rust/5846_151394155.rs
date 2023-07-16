 rust
macro_rules! foo {
    ($x:tt) => (
        macro_rules! internal {
            () => { type t = $x<i8>; }
        }
        internal!();
    )
}

foo!(Box);
