 Rust
impl<T> Foo<T> {
    fn foo(&self) {
        println!("generic");
    }
}

impl<T> Foo<T> {
    fn foo(&self) {
        println!("specific");
    }
}
