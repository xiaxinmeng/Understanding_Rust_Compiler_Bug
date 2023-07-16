rust
struct Foo<T>(T);

impl<T> Foo<T> {
    const S: & str = ...;
//            ^
//            the only usable lifetime here is `'static`, so we might as well use it.
}
