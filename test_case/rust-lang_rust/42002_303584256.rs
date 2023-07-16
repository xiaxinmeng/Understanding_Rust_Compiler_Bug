rust
unsafe trait TrustedRead: Read {}

trait Read: InitBuffer { ... }

pub unsafe trait InitBuffer {
    fn init_buffer(&self, buf: &mut [u8]);
}

unsafe impl<T> InitBuffer for T {
    default fn init_buffer(&self, buf: &mut [u8]) {
        for b in buf { *b = 0; }
    }
}

unsafe impl<T: TrustedRead> InitBuffer for T {
    fn init_buffer(&self, buf: &mut [u8]) {}
}
