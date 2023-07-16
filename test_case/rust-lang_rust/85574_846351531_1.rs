rust
enum Foo<'a> {
    Var(&'a str)
}

impl Foo<'_> {
    fn f(other: Foo<'_>) {
        match other {
            Self::Var(..) => {}
        }
    }
}
