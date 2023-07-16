rust
pub trait ViewFn<S> {
    type Output<'ast>: Sized;
    fn view<'ast>(&self, node: &'ast S) -> Self::Output<'ast>;
    // ...
}
