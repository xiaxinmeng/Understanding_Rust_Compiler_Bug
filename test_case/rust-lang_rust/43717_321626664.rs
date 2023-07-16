rust
// Can be cast to `MyAdaptor<Fn(u8) -> u32>`.
struct MyAdaptor<F: ?Sized> {
    ...,
    inner: ::std::iter::Map<::std::ops::Range<u8>, F>,
}
