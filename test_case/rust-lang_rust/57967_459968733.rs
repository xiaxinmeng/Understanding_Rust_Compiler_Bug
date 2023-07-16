rust
fn size_of_val<T>(_: &T) -> usize {
    std::mem::size_of::<T>()
}

struct Foo(i64);

pub fn size_of_foo() -> usize {
    size_of_val(&Foo(0))
}
pub fn size_of_foo_ctor() -> usize {
    size_of_val(&Foo)
}
