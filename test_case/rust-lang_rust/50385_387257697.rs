rust
macro_rules! m {
    (T: $i:ident) => { true };
    (T: $t:tt) => { false } // or $l:lifetime but it's not needed to see breakage from expanding :ident
}

m!(T: 'static)
