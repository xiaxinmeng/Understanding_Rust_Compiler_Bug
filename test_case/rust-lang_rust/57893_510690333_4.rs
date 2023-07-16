rust
impl<A, B> Object<B> for A
where
    B: ?Sized,
{
    type Output = B;
}
