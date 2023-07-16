rust
struct Foo<T>(T);

impl<'a, T: 'a> Foo<T> {
    const FUNC: &'a fn() -> T = {
        let func: fn() -> T = || loop {};
        &{ func }
    };
}
