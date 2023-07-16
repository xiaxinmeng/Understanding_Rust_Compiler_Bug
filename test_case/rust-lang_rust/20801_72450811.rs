 rust
pub struct Foo;
pub fn destroy(handle: *mut Foo) {
    unsafe {
        *handle;
    }
}
