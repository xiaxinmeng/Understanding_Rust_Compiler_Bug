rust
#![feature(generic_const_exprs)]
struct Foo<T, const N: usize>([u8; N + 1]) where [u8; N + 1]: Trait<T>;
trait Trait<T> {}
impl<T, U> Trait<T> for U {}
