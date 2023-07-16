rust
pub trait Read: InitBuffer {
    ...

    // is this actually required?
    fn init_buffer(&self, buf: &mut [u8]) {
        <Self as InitBuffer>::init_buffer(self, buf);
    }
}

pub unsafe trait InitBuffer {
    fn init_buffer(&self, buf: &mut [u8]);
}

unsafe impl<T> InitBuffer for T {
    default fn init_buffer(&self, buf: &mut [u8]) {
        for b in buf { *b = 0; }
    }
}
