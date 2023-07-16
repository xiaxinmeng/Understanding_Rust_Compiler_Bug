rust
trait Foo<A> {}
struct Wrap<F>(F);

impl<A: Clone, F: Foo<A>> Wrap<F> {}

pub fn main() {}
