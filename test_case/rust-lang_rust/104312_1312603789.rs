
trait Foo<A#1, A#2 = A#3> {}
type Bar<T> = dyn Foo<T>;
fn main() {}
