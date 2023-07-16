rust
macro_rules! after_otc {
    ($($e:literal) some tok *) => {
        0 $(+ $e)*
    }
}
