rust
trait Foo {
    type Bar;
}
fn foo<T: Foo>() {
    std::mem::size_of::<T::Bar>();
}
fn main() {}
