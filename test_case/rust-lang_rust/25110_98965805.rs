 Rust
trait ImplMe<'a, A> {}
impl<'a, A> ImplMe<'a, A> for A where A : 'a {}
