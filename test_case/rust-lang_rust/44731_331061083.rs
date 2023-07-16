rust
#![feature(conservative_impl_trait)]
fn foo() -> Box<impl Clone> { //~ ERROR E0283
    loop {}
}
fn main() {
}
