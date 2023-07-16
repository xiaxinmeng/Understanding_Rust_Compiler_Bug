 Rust
#![feature(unboxed_closures)]
struct Foo<Ret> (Ret);
impl<T> FnMut()->T for Foo<T> {
    extern "rust-call" fn call_mut(&mut self, _: ())->T { self.0 }
}
fn main() {}
