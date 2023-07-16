rust
#![feature(decl_macro)]

macro m() {
    let v = Vec::<u8>::new(); // OK
}

fn main() {
    m!();
}
