rust
trait Trait {}
impl<T> Trait for fn(&T) { }
impl<T> Trait for fn(T) { } // still accepted, but warns
