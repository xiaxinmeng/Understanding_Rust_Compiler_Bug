
impl<T> A::Trait for Box<T> { } // in crate A
impl<T> A::Trait for &T { } // in crate A
