rust
#[deny(single_use_lifetimes)]
#[derive(Eq)]
struct Foo<'a, T> {
    _y: &'a mut T,
}
impl<T> PartialEq for Foo<'_, T> {
    fn eq(&self, _x: &Self) -> bool {
        true
    }
}
