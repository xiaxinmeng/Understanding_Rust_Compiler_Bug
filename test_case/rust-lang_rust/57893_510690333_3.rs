rust
impl<A, B> Object<A> for dyn Object<A, Output = B> {
    type Output = B;
}
