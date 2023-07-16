rs
pub struct Foo<T> {
    foo: u64,
    bar: T,
}

pub fn test(x: (i32, i32)) -> Foo<(i32, i32)> {
    Foo { foo: 0, bar: x }
}
