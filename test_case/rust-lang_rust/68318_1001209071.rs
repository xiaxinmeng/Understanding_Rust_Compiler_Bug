rust
trait A {}
trait B {}

// logically equivalent negative impls
impl<T: A> !B for T {}
impl<T: B> !A for T {}

// this should not be possible, but compiles:
impl A for () {}
impl B for () {}
