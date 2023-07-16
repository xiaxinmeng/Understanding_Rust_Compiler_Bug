 Rust
pub trait Foo { type Output: 'static; }
pub trait Bar<P> {}
pub trait Baz<P> {}

impl<T: 'static, W: Foo<Output=T>> Baz<*mut T> for W {}

impl<P, T: Baz<P>> Bar<P> for T {}
impl<T> Bar<*mut T> for [T; 1] {}

fn main() {}
