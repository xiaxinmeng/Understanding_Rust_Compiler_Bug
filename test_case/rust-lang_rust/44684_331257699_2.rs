rust
#[derive(Clone)]
enum Foo<'a> {
    Bar(&'a str),
}

impl<'a> Foo<'a> {
    fn bar(&self, other: Foo<'a>) -> Foo<'a> {
                         // ^^^^ note the explicit lifetime here
        match *self {
            Foo::Bar(s) => {
                if s == "test" {
                    other
                } else {
                    self.clone()
                }
            }
        }
    }
}
