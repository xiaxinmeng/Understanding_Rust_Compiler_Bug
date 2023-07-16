rust
trait A<P> {}
trait B<P> {}

impl<P> B<P> for i32 {}

impl<P, T: A<P>> B<P> for T {}
