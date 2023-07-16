rust
#![feature(decl_macro)]

struct S;

mod m {
    macro m() {
        let s = crate::S; // OK
    }
    
    fn check() {
        m!();
    }
}

fn main() {}
