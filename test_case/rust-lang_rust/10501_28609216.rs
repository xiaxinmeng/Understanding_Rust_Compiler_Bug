
impl<A,R> Clone for fn(A) -> R {
    fn clone(&self) -> fn(A) -> R { *self }
}
