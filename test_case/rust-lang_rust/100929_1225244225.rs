rust
fn main() {
    macroname!();
    macroname2!();
}
#[macro_export]
macro_rules! macroname {
    ( $( $x:expr),* ) => {}
}
macro_rules! macroname2 {
    ( $( $x:expr),* ) => {}
}
