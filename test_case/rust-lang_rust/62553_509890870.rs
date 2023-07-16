rust
fn into_inner(self) -> Box<[u8]> {
    unsafe {
        let result = std::mem::MaybeUninit::new(std::ptr::read(&self.inner));
        std::mem::forget(self);
        result.assume_init()
    }
}
