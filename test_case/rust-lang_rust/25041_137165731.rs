 Rust
#![feature(unboxed_closures,core)]
use std::mem;

trait Foo {
    fn foo(&self) -> usize;
}
impl<F, A> Foo for F where F: FnOnce(A) {
    fn foo(&self) -> usize { mem::size_of::<A>() }
}

struct S;
impl FnOnce<(u32,)> for S {
    type Output = ();
    extern "rust-call" fn call_once(self, _args: (u32,)) {}
}
impl FnOnce<(u8,)> for S {
    type Output = ();
    extern "rust-call" fn call_once(self, _args: (u8,)) {}
}
fn main() {
    println!("{}", <S as Foo>::foo(&S)); // which impl is used?
}
