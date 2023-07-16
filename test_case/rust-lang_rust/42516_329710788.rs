rust
struct Bar<'a, T: 'a>(&'a mut T);
struct Foo<'a, T: 'a>(&'a mut Bar<'a, T>);

fn make_foo<'a, T>(m: &'a mut Bar<T>) -> Foo<'a, T> {
    Foo(m)
}
