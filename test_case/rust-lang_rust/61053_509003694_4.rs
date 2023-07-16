rust
macro_rules! foo {
    ($($k:tt $v:tt)*) => {
        macro_rules! bar {
            // The lint does not recognize the repetition as a body, thus falls back
            // to checking as if outside a nested macro definition, and thus reports
            // $f as free.
            $(($f:tt $k) => { $f($v) };)*
        }
    };
}
