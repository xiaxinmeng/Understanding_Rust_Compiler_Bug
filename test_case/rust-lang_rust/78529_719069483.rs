rust
pub struct D<A, B> {
    a: *mut Option<A>,
    b: *mut Vec<B>,
}

impl<A, B> Drop for D<A, B> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self.a);
            std::ptr::drop_in_place(self.b);
        }
    }
}
