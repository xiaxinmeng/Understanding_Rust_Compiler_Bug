 rust
impl Eq for Foo {
    fn eq(&self, other: &Foo) -> bool {
        match self {
            Foo1 => match other {
                Foo1 => true,
                _ => false
            },
            Foo2 => match other {
                Foo2 => true,
                _ => false
            }
            // ...
        }
    }
}
