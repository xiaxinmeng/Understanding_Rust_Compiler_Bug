rust
fn inner<'a, F: FnOnce() -> Foo<'a>>(f: F) -> [Foo<'a>; 2] {
    [f(); 2]
}

struct NeverCopy<T>(T);
impl<T> NeverCopy<T> {
    fn inner(self) -> T {
        self.0
    }
}

fn duplicate<'a>(foo: NeverCopy<Foo<'a>>) -> [Foo<'a>; 2] {
    inner(move || foo.inner())
}
