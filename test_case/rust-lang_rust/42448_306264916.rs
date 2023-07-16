rust
#![feature(decl_macro)]
macro m($f:ident) {
    #[derive(Debug)]
    struct S;
    fn $f() { println!("{:?}", S); }
}

fn main() {
    struct S; // This does not conflict with `S` from the macro
    m!(f);
    f();
}
