 Rust
pub trait Foo<T> {
    fn foo(&self); // no places to differ, trait can not have duplicates
}

// must differ in concrete type both by T and U
pub trait Bar<T, U> {
    fn bar1<T>(&mut self, t: &T); // must differ in concrete type by T
    fn bar2<U>(&mut self, u: &U); // must differ in concrete type by U
}

// must differ in concrete type by T or U
pub trait Baz<T, U> {
    fn baz<T, U>(&self, t: &T, u: &U);
}
