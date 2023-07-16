 rust
struct Foo<T = ()>(T);

impl<T> Foo<T> {
    fn bar() {}
}

fn main() {
    Foo::bar();
    // Equivalent to:
    <Foo<_>>::bar();
}
