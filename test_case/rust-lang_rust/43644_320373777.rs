rust
pub fn do_something<'a, F>(&'a self, foo: F) where FooRef<'a>: From<F> {
    let foo_ref: FooRef = foo.into();
    self.do_something_internal(foo_ref)
}

pub fn do_something_internal(&self, _foo: FooRef) {
    let _self_ref: FooRef = self.into();
    // ... 
}
