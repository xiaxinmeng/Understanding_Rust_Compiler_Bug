rust
struct Foo<'a>(&'a ());

impl<'a> Foo<'a> {
    async fn named(arg: &'a ()) -> Foo<'a> {
        Self(arg)
    }
    
    async fn selfie(arg: &'a ()) -> Self {
        Self(arg)
    }
}
