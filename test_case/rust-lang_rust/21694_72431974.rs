 rust
struct Pair<A, B>(A, B);

trait Foo: Sized {
    type Item = ();
    fn foo<B>(self, other: B) -> Pair<Self, B> {
        Pair(self, other)
    }
}
impl Foo for () {
    type Item = ();
}
#[cfg(equality)]
impl<A: Foo, B: Foo<Item = A::Item>> Foo for Pair<A, B> {
    type Item = A::Item;
}
#[cfg(not(equality))]
impl<A: Foo, B: Foo> Foo for Pair<A, B> {
    type Item = A::Item;
}

fn main() {
    ().foo(())
        .foo(())
        .foo(())
        .foo(())
        .foo(())
        .foo(())
        .foo(())
        .foo(())
        .foo(())
        ;
}
