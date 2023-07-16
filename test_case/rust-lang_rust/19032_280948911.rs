
trait A {}
trait B {}
impl B for i32 {}
impl<T: A> B for T {}
