rust
pub enum Foo<'string> {
    String(&'string str),
}

impl Foo<'_> {
    fn f(&self, other: &Foo<'_>) {
        match (self, other) {
            // Important: a and b swap between the two patterns, and one pattern uses `Self`
            (Foo::String(a), Foo::String(b))
            | (Self::String(b), Foo::String(a)) => {}
        }
    }
}
