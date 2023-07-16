rust
impl<T> Cake2 for T where T: Cake + Foo<Error = i32> + Bar<Error = u64> {}
