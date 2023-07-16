rust
#![crate_type = "lib"]
struct Foo;
pub struct Path<T: Bar> {
    inner: T::Slice,
}
pub trait Bar: Sized {
    type Slice: ?Sized;
    fn open(_: &Path<Self>);
}
impl Bar for Foo {
    type Slice = ([u8],);
    fn open(_: &Path<Self>) {
        unimplemented!()
    }
}
