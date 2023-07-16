rust
fn upcast<Dyn: ?Sized + Unsize<dyn Foo>>(bar: &Dyn) -> &dyn Foo { bar }
