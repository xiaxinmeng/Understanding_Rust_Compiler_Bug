rust
struct Foo<'a, A> {
    inner: &'a A,
}

struct Bar<'b, B> {
    inner: &'b B,
}

impl<'a, 'b, B> Foo<'a, Bar<'b, B>>
{
    fn inner(&self) -> &'a Bar</*'b,*/ B> {
        self.inner
    }
}

fn func<'a, 'b, B>(foo: Foo<'a, Bar<'b, B>>) {
    let _: &Bar<'b, B> = foo.inner();
}
