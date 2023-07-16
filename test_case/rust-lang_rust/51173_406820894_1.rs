 rust
macro_rules! working {
    ( $doc:expr, $item:item ) => { #[doc = $doc] $item }
}

working! {
    stringify!(Working),
    enum Working { }
}
fn main() {}
