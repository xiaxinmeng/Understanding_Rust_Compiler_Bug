 rust
struct Struct<A>(A);
trait ImplMe<'a, A> {}
impl<'a, A> ImplMe<'a, A> for bool where Struct<A> : 'a {}
