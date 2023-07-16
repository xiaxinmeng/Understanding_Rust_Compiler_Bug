 rust
struct Foo { data: *mut i8, len: uint }
impl Foo {
    fn as_slice(&self) -> &[i8] {
        unsafe {
            // value "self.data as *const i8" does not live long enough
            std::slice::from_raw_buf(&(self.data as *const i8), self.len)
        }
    }
}
