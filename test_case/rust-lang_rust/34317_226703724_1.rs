 rust
macro_rules! m {
    ($x:ident) => {
        struct S;
        impl S {
            fn f(&self) {
                self; // this is OK, but
                $x; // this is a name resolution error (since it came from outside the macro)
            }
        }
    }
}
fn main() {
    m!(self); // (in this expansion)
}
