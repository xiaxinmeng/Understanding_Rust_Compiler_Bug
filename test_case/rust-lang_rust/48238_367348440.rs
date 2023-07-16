rust
trait Fn<A> {
    type Output;
    fn call<'a>(&'a self, args: A) -> Self::Output;
}
