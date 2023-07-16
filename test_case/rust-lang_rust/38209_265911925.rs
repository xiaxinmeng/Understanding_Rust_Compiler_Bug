rust
struct Foo(i32);

trait Bar<A> { type B; }

trait Baz {}

impl<A, B: Bar<Foo, B=A>> Baz for B {}

fn main() {}
