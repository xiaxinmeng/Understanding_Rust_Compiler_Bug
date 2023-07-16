rust
struct A;
struct B;
trait Foo<T> {}
impl Foo<B> for A {}

fn main() { g::<_>(B); }

fn g<T>(_: T) where A: Foo<T> {}
