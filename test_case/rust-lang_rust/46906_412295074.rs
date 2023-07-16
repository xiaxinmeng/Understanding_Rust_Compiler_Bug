Rust
pub struct MyType;

impl MyType {
    pub fn offset(self: *mut Self, _offset: usize) -> *mut i32 {
        /* .. */
        1 as *mut i32
    }
}
