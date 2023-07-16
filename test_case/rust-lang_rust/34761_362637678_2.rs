rust
struct Transmuted<T> {
    data: Vec<u8>,
    marker: PhantomData<T>,
}
impl<T> Transmuted<T> {
    fn new(x: T) { ... horrible mem::transmute to store the byte representation of x in data }
}
impl<T> Deref for Transmuted<T> {
    type Target = T;
    fn deref(&self) -> &T { ... more horrible transmuting ... }
}
impl<#[may_dangle] T> Drop for Transmuted<T> {
    fn drop(&mut self) { ... what can we do with data here? ... }
}
