rust
#[derive(Default)]
pub struct S<A>(pub A);

impl<'a, A> Drop for S<&'a A> {
    fn drop(&mut self) {}
}
