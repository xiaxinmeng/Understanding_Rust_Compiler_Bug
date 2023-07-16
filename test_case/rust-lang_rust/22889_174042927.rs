 rust
struct Foo;
impl Foo {
    fn id() {}
}
impl Foo {
    fn id() {}
}

struct Bar<T>(T);
impl<T> Bar<T> {
    fn bar(&self) {}
}
impl Bar<i32> {
    fn bar(&self) {}
}
