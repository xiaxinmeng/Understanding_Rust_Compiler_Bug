 rust
fn f<'a, Y: ?Sized+'static>(y: &'a Y) -> Z<Y> {
    Z { _data: std::marker::PhantomData }
}
