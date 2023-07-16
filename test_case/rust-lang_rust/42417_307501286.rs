rust
fn foo<'a, A, B>(x: &'a A) -> &'a B { ... }

struct Foo<'a, A> {
    data: PhantomData<fn(&'a A) -> &'a B>
}
