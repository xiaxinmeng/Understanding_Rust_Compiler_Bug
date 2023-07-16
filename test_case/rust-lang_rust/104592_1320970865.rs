rust
trait Foo {
    async fn foo(&self, x: &u32);
}

impl<'blah> Foo for &'blah () {
    // Accepted: Matches desugared form (these are equivalent)
    fn foo<'me, 'x>(&'me self, x: &'x u32) -> impl Future<Output = ()> + 'me + 'x { async {} }
    fn foo<'x>(&'me self, x: &'x u32) -> impl Future<Output = ()> + '_ + 'x { async {} }

    // Not accepted: return type must include all input lifetimes
    fn foo<'x>(&self, x: &'x u32) -> impl Future<Output = ()> + '_ { async {} }
    fn foo<'x>(&self, x: &'x u32) -> impl Future<Output = ()> + 'x { async {} }
    fn foo<'x>(&self, x: &'x u32) -> impl Future<Output = ()> { async {} }

    // Not accepted: return type must not include additional lifetimes, like `'static`
    fn foo<'x>(&self, x: &'x u32) -> impl Future<Output = ()> + '_ + 'x + 'static { async {} }

    // Not accepted (this already works): return type must specify `Output`
    fn foo<'x>(&self, x: &'x u32) -> impl Future + '_ + 'x { async {} }

    // This one is acceptable because the extra `+ 'blah` bound is already implied by `+ '_`
    // (anything that outlives a reference to `self` must also outlive `self`),
    // but it's also okay to give an error here for now.
    fn foo<'x>(&self, x: &'x u32) -> impl Future<Output = ()> + '_ + 'x + 'blah { async {} }
}
