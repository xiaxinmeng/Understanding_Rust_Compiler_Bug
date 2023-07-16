rust
macro_rules! after_otc {
    ($($e:literal) | *) => {
        0 $(+ $e)*
    }
}

macro_rules! after_otc_tok {
    ($($e:literal) tok *) => {
        0 $(+ $e)*
    }
}
