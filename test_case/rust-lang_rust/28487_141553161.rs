 Rust
fn foo<T>() {} // this requires `T: Sized`
trait Foo<P> {} // this requires `P: Sized`, but not `Self: Sized`
impl<T> Foo<u32> for [*const T] {} // this requires `T: Sized` (but not `[*const T]: Sized`)
impl<T:?Sized> Foo<()> for T {} // does not require `T: Sized`
