 rust
trait X<T> {}

impl X<()> for () {}

#[cfg(equality)]
impl<A: Foo, B: Foo> Foo for Pair<A, B>
        where A::Item: X<B::Item> {
    type Item = A::Item;
}
