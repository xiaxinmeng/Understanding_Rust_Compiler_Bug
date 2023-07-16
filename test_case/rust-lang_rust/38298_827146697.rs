rust
trait A { type Q; }
trait B where Self: A, <Self as A>::Q: Sync {}
fn foo<T: B>() {} // error
