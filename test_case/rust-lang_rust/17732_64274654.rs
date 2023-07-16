 Rust
#![feature(associated_types)]
trait Bindable {
    type Ret;
}
trait Foo<A> where A: Bindable {
    fn func<B>(b: B) -> <A as Bindable>::Ret;
}
fn main() {}
