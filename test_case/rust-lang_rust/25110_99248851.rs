 rust
struct Struct<A>(A);
trait ImplMe<A> {
    fn impl_me(&self) {}
}

impl<'a, A> ImplMe<A> for bool where Struct<A> : 'a {}

fn main() {
    true.impl_me();
}
