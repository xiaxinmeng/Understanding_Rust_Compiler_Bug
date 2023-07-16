rust
struct Foo<T>(T);

impl<T> Foo<T> {
    const FUNC: &'static fn()->T = &||loop{};
}
