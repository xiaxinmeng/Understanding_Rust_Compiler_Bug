rust
trait Foo<A, A = A> {}
type Bar<T> = dyn Foo<T>;
fn main() {}
