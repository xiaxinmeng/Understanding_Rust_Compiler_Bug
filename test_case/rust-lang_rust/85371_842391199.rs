rust
pub trait Foo {}
pub trait Bar<T>: Foo {}
pub trait Baz<T> {}

impl<U, T> Baz<U> for T where T: Bar<U> {}
//impl<U, T> Baz<U> for T where T: Bar<U> + Foo {}

impl<U, T> Baz<U> for Option<T> where T: Baz<U> {}
