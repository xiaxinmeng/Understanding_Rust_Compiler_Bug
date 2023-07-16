 rust
impl Box<Any> {
    pub fn downcast<T: Any>(self) -> Result<Box<T>, Box<Any>> { ... }
}
impl Box<Any + Send> {
    pub fn downcast<T: Any>(self) -> Result<Box<T>, Box<Any + Send>> { ... }
}

impl Foo<i32> {
   fn abc(&self) { }
}
impl Foo<u32> {
   fn abc(&self, flag: bool) { }
}
