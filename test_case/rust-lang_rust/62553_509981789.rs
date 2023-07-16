rust
fn into_inner(self) -> Box<[u8]> {
    use ::std::{mem::MaybeUninit, ptr};
    unsafe {
        type T = Box<[u8]>;
        let inner = ptr::read(&mut self.inner as *mut T as *const MaybeUninit<T>);
        std::mem::forget(self);
        inner.assume_init()
    }
}
