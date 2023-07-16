 rust
struct Struct<A>(A);
trait ImplMe<A> {}
impl<'a, A> ImplMe<A> for bool where Struct<A> : 'a {}
