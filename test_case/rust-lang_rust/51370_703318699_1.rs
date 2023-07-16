rust
struct Foo<T>(T);

impl<'a, T : 'a> Foo<T> {
    const S: & str = ...;
//            ^
//            two applicable lifetimes in scope, here: `'a` and `'static`; do not favor any.
}
