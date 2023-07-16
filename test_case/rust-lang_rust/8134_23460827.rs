 rust
struct Foo;
impl<T> Foo {
    pub fn foo(&self) {
        static X: uint = 1;
    }
}
