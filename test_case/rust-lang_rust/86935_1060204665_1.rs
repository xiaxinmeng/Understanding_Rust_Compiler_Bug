rust
struct MyClosure;
impl<T> FnOnce<(T,)> for MyClosure {
    type Output = ();
    ...
}
