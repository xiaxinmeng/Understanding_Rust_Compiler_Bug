rust
//- dep/lib.rs
struct Foo;
impl Debug for Foo { /* ... */ }
impl Display for Foo { /* ... */ } // <- this is added

//- my/lib.rs
struct Bar {
    foo: Foo
}
impl Debug for Bar {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.foo.fmt(f) // <- and now this fails to compile
    }
}