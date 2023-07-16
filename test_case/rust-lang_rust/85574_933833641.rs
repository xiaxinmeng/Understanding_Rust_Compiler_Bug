rust
pub enum Foo<'string> {
    String(&'string str),
}

impl<'a> Foo<'a> {
    fn f<'b>(&self, other: &Foo<'b>) {
        match (self, other) {
            // Important: a and b swap between the two patterns, and one pattern uses `Self`
            (Foo::String(a), Foo::String(b))
            | (Foo::<'a>::String(b), Foo::String(a)) => {}
        }
    }
}
