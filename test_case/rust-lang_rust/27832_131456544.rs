 rust
macro_rules! m {
    ( $b:expr ) => ();
    ( $t:tt $u:tt ) => ();
}

fn main() {
    m!(3);      // works trivially
    m!(1 2);    // works, since `1` is a valid expression
    m!(_ 1);    // doesn't work, since `_` is not an expression (but a valid TT, of course)
}
