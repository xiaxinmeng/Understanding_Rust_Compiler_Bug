 rust
macro_rules! m {
    ($x:ident) => {
        let x = 1; // This binding can only be used by the macro, so
        x; // this is OK, but
        $x; // this is a name resolution error (since it came from outside the macro)
    }
}
fn main() {
    m!(x); // (in this expansion)
}
