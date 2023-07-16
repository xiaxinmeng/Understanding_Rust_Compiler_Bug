rust
pub fn collect(_: impl IntoIterator<Item = impl for<T: AsRef<[u8]>> Borrow<Data<T>>>) {
    unimplemented!()
}